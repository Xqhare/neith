use std::{io::{self, Error}, path::{Path, PathBuf}, time::Instant, sync::{Mutex, MutexGuard}, rc::Rc};

use chrono;

use crate::{utils::util::{*}, data::Data};

#[cfg(test)]
mod tests;

mod success;
// The column representation
mod column;
// The table representation
mod table;
// The general data representation
mod data;
// My util module.
mod utils;

use crate::table::Table;
use crate::utils::jisard;
use jisard::read_json_from_neithdb_file;
use utils::jisard::write_neithdb_file;
use success::Success;

#[derive(Clone, Debug)]
pub struct Neith {
    path: PathBuf,
    ram_mode: bool,
    job_history: bool,
    job_history_table_index: Option<usize>,
    // putting tables on the heap, as they could grow quite large! Pointing to it also makes sense,
    // along being needed for clone.
    tables: Vec<Box<Rc<Mutex<Table>>>>,
    split_pattern: String,
}

impl Default for Neith {
    fn default() -> Self {
        let tables: Vec<Box<Rc<Mutex<Table>>>> = Default::default();
        let ram_mode = true;
        let job_history = false;
        let job_history_table_index = None;
        let path = PathBuf::new();
        let split_pattern = ",+".to_string();
        return Neith{ tables, path, ram_mode, job_history, job_history_table_index, split_pattern};
    }
    
}

impl From<PathBuf> for Neith {
    fn from(value: PathBuf) -> Self {
        let path = canonize_path(value);
        let read_file = read_json_from_neithdb_file(path.clone());
        let mut tables: Vec<Box<Rc<Mutex<Table>>>> = Default::default();
        let ram_mode = false;
        let job_history = false;
        let job_history_table_index = None;
        let split_pattern = ",+".to_string();
        for table in read_file.entries() {
            let table = Box::new(Rc::new(Mutex::new(Table::from(table))));
            tables.push(table);
        }
        return Neith{ tables, path, ram_mode, job_history, job_history_table_index, split_pattern};
    }
}

impl Neith {
    /// Creates a new Neith instance, with no contents.
    /// For general use, `connect(filename)` is highly recommended.
    pub fn new(value: PathBuf, ram_mode: bool, job_history: bool) -> Self {
        let path = canonize_path(value);
        let tables: Vec<Box<Rc<Mutex<Table>>>> = Default::default();
        let job_history_table_index = None;
        let split_pattern = ",+".to_string();
        return Neith{ tables, path, ram_mode, job_history, job_history_table_index, split_pattern};
    }
    /// Creates the connection to your database. Most if not all programs will start with this.
    /// ```
    /// use neith::Neith;
    /// let con = Neith::connect("myDBname");
    /// ```
    pub fn connect<P>(filename: P) -> Self where P: AsRef<Path> + Clone, PathBuf: From<P> {
        let path = canonize_path(filename.into());
        if check_for_persistant_db(path.clone()) {
            let connection = Neith::from(path);
            return connection;
        } else {
            let connection = Neith::new(path, false, false);
            return connection;
        }
    }
    /// Connect in ram mode. No way to save even if you want to!
    pub fn connect_ram_mode(job_history: bool) -> Self {
        let mut connection = Neith::default();
        let _ = connection.set_job_history(job_history);
        return connection;
    }
    /// A toggle for job-history, set to true to record, set to false to not record.
    pub fn set_job_history(&mut self, value: bool) -> Result<Success, Error> {
        self.job_history = value;
        if self.exists_table("job_history".to_string())? && self.job_history {
            let index = self.search_for_table("job_history".to_string()).unwrap();
            self.job_history_table_index = Some(index);
            return Ok(Success::SuccessMessage(value));
        } else if !self.exists_table("job_history".to_string())? && self.job_history {
            let table_columns: Vec<(String, bool)> = vec![("id".to_string(), true), ("command".to_string(), false), ("time".to_string(), false), ("duration".to_string(), false)];
            let table_prop = ("job_history".to_string(), table_columns);
            let job_history_table = Box::new(Rc::new(Mutex::new(Table::from(table_prop))));
            self.tables.push(job_history_table);
            // Table len == number of elements. element 1 == index 0
            self.job_history_table_index = Some(self.tables.len().saturating_sub(1));
        }
        return Ok(Success::SuccessMessage(value));
    }
    pub fn set_marker(&mut self, split_pattern: &str) {
        self.split_pattern = split_pattern.to_string();
    }
    /// Saves the current state of the database to disc.
    pub fn save(self) -> Result<Success, json::JsonError> {
        return write_neithdb_file(self);
    }
    /// Makes Neith persistant and saves it's current state at the supplied Path. This path needs
    /// to be valid.
    ///
    /// ## Errors
    /// Will error if supplied with an incorrect path or if Neith does not have write permission at
    /// the supplied path.
    pub fn make_persistant(&mut self, save_path: PathBuf) -> Result<Success, json::JsonError> {
        self.path = save_path;
        self.ram_mode = false;
        return self.clone().save();
    }
    // This is the general apperance of a mk_table call.
    // mk_table(table_name, column_vec((column_name0, unique_bool, type)), (column_name1, unique_bool, type))
    //
    // alt:
    // mk_table({table_name} with {collum_name}[uniqe_bool])
    //
    // I need a table, column from(String) where I decode.

    // instead of having all these functions, have a .execute() and do more of the text comprehension planned already.
    //
    // execute(new table 'tablename' with ('rowname' 'unique', 'other_rowname' 'unique'))
    // execute(new row 'tablename' with ('rowname' 'unique', 'other_rowname' 'unique'))
    // execute(new data 'tablename' (columnname0, columnname1, ...) values (val0, val1, ...))
    // execute(delete table with 'tablename')
    // execute(delete row with 'rowname' in 'tablename') -> HAS TO FAIL IF UNIQUE!!!!
    // execute(update 'tablename' where ['columnname' = 'data', {and/not/or} 'other_columnname' = 'other data'] with ('a_columnname' = 'a_data', 'other_columnname' = 'new_data'))
    // execute(select 'rowname' and 'other_rowname' from 'tablename' where ['columnname' = 'data', {and/not/or} 'other_columnname' = 'other data'])
    // execute(select * from 'tablename' where ['columnname' = 'data', {and/not/or} 'other_columnname' = 'other data'])
    //
    // As `get` is NOT sql syntax (at least as far as I know), I will use it here for my helper functions.
    // execute(get min in 'rowname' from 'tablename') -> Meaning the minimum value in any column entry.
    // execute(get max in 'rowname' from 'tablename') -> Meaning the maximum value in any column entry.
    // execute(get len of 'tablename') -> Meaning the amount of rows.
    //
    // For returning the query or a success message, I could wrap another custom wrapper in Result,
    // e.g. Result<NeithAnswer, io::Error>

    /// Execute is the main function for interaction. For the query syntax in nql please consult
    /// the readme.
    ///
    /// ## Returns
    /// This function will always return someting, be it a simple `SuccessMessage` to let you know,
    /// or your requested data.
    ///
    /// ## Errors
    /// This function has many different ways to error. Please read the error message carefully, as
    /// it contains important information in most cases.
    pub fn execute(&mut self, query: &str) -> Result<Success, io::Error> {
        // Conditional variables for job_history
        let start = Instant::now();
        let date = chrono::Utc::now().to_rfc3339();
        // Real execute starts here:
        let binding = Into::<String>::into(query);
        let command_lvl1 = strip_leading_word(binding.clone());
        match command_lvl1.0.as_str() {
            "new" => {
                let command_lvl2 = strip_leading_word(command_lvl1.1);
                let command_lvl3 = strip_leading_word(command_lvl2.1);
                let tablename =  command_lvl3.0;
                match command_lvl2.0.as_str() {
                    "table" => {
                        let command_lvl4 = strip_leading_word(command_lvl3.1);
                        if command_lvl4.0.as_str().contains("with") {
                            if self.search_for_table(tablename.clone()).is_ok() {
                                // Table exists already; Don't do anything act like everything is
                                // fine!
                                return Ok(Success::SuccessMessage(true));
                            }
                            let columns = decode_columnmaker(command_lvl4.1).unwrap();
                            let answ = Box::new(Rc::new(Mutex::new(Table::from((tablename, columns)))));
                            self.tables.push(answ);
                            if self.job_history {
                                let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                if history_table.is_ok() {
                                    let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                }
                            }
                            // Successful decoding of syntax!
                            return Ok(Success::SuccessMessage(true));
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "column" => {
                        let command_lvl4 = strip_leading_word(command_lvl3.1);
                        if command_lvl4.0.as_str().contains("with") {
                            let columns = decode_columnmaker(command_lvl4.1).unwrap();
                            let table_index = self.search_for_table(tablename)?;
                            let table = self.tables[table_index].lock();
                            if table.is_ok() {
                                let mut ok_table = table.unwrap();
                                let answ = ok_table.new_columns(columns);
                                // Successful decoding of syntax!
                                if answ == Success::SuccessMessage(true) {
                                    if self.job_history {
                                        let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                        if history_table.is_ok() {
                                            let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                        }
                                    }
                                    return Ok(answ);
                                } else {
                                    return Err(Error::other("Invalid nql syntax."));
                                }
                            } else {
                                return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                            }
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "data" => {
                        let command_lvl4 = command_lvl3.1;
                        let decoded = decode_list_columndata(command_lvl4, self.split_pattern.clone());
                        let table_index = self.search_for_table(tablename)?;
                        let table = self.tables[table_index].lock();
                            if table.is_ok() {
                                let mut ok_table = table.unwrap();
                                let answ = ok_table.new_data(decoded)?;
                                // Successful decoding of syntax!
                                if answ == Success::SuccessMessage(true) {
                                    if self.job_history {
                                        let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                        if history_table.is_ok() {
                                            let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                        }
                                    }
                                    return Ok(answ);
                                } else {
                                    return Err(Error::other("Invalid nql syntax."));
                                }
                            } else {
                                return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                            }

                    },
                    _ => return Err(Error::other("Invalid nql syntax.")),
                }
            },
            "delete" => {
                let command_lvl2 = strip_leading_word(command_lvl1.1);
                match command_lvl2.0.as_str() {
                    "table" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1);
                        if command_lvl3.0.as_str().contains("with") {
                            let tablename = command_lvl3.1;
                            let answ = self.delete_table(tablename);
                            if answ.is_ok() {
                                if self.job_history {
                                    let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                    if history_table.is_ok() {
                                        let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                    }
                                }
                                return Ok(answ.unwrap());
                            } else {
                                return Err(Error::other("Invalid nql syntax."));
                            }
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "column" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1);
                        if command_lvl3.0.as_str().contains("with") {
                            let command_lvl4 = strip_leading_word(command_lvl3.1);
                            let columnname = command_lvl4.0;
                            let command_lvl5 = strip_leading_word(command_lvl4.1);
                            if command_lvl5.0.as_str().contains("in") {
                                let tablename = command_lvl5.1;
                                let answ = self.delete_column(tablename, columnname);
                                if answ.is_ok() {
                                    if self.job_history {
                                        let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                        if history_table.is_ok() {
                                            let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                        }
                                    }
                                    return Ok(answ.unwrap());
                                } else {
                                    return Err(Error::other("Invalid nql syntax."));
                                }
                            } else {
                                return Err(Error::other("Invalid nql syntax."));
                            }
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "data" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1);
                        if command_lvl3.0.as_str().contains("in") {
                            let command_lvl4 = strip_leading_word(command_lvl3.1);
                            let tablename = command_lvl4.0;
                            let table_index = self.search_for_table(tablename)?;
                            let command_lvl5 = strip_leading_word(command_lvl4.1);
                            if command_lvl5.0.as_str().contains("where"){
                                let conditions = command_lvl5.1;
                                let finished_search = self.search_conditionals(conditions, table_index)?;
                                let table = self.tables[table_index].lock();
                                if table.is_ok() {
                                    let mut ok_table = table.unwrap();
                                    let answ = ok_table.delete_data(finished_search)?;
                                    // Successful decoding of syntax!
                                    if answ == Success::SuccessMessage(true) {
                                        if self.job_history {
                                            let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                            if history_table.is_ok() {
                                                let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                            }
                                        }
                                    return Ok(answ);
                                    } else {
                                        return Err(Error::other("Invalid nql syntax."));
                                    }
                                } else {
                                    return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                                }
                            } else {
                                return Err(Error::other("Invalid nql syntax."));
                            }
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    _ => return Err(Error::other("Invalid nql syntax.")),
                }
            },
            "update" => {
                let command_lvl2 = strip_leading_word(command_lvl1.1);
                let tablename = command_lvl2.0;
                let command_lvl3 = strip_leading_word(command_lvl2.1);
                if command_lvl3.0.as_str().contains("where") {
                    let command_lvl4 = strip_condition_list(command_lvl3.1);
                    let conditions = command_lvl4.0;
                    let command_lvl5 = strip_leading_word(command_lvl4.1);
                    if command_lvl5.0.as_str().contains("with") {
                        let decoded_list = decode_list_columndata(command_lvl5.1, self.split_pattern.clone());
                        let table_index = self.search_for_table(tablename)?;
                        let search = self.search_conditionals(conditions, table_index)?;
                        let table = self.tables[table_index].lock();
                            if table.is_ok() {
                                let mut ok_table = table.unwrap();
                                let answ = ok_table.update_data(decoded_list, search)?;
                                // Successful decoding of syntax!
                                if answ == Success::SuccessMessage(true) {
                                    if self.job_history {
                                        let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                        if history_table.is_ok() {
                                            let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                        }
                                    }
                                    return Ok(answ);
                                } else {
                                    return Err(Error::other("Invalid nql syntax."));
                                }
                            } else {
                                return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                            }
                    } else {
                        return Err(Error::other(format!("Invalid nql syntax. {:?} should be 'where'.", command_lvl3.0)));
                    }
                } else {
                    return Err(Error::other(format!("Invalid nql syntax. {:?} should be 'where'.", command_lvl3.0)));
                }
            },
            "select" => {
                let command_lvl2 = strip_column_list(command_lvl1.1)?;
                let command_lvl3 = strip_leading_word(command_lvl2.1);
                if command_lvl3.0.as_str().contains("from") {
                    let command_lvl4 = strip_leading_word(command_lvl3.1);
                    let tablename = command_lvl4.0;
                    let table_index = self.search_for_table(tablename)?;
                    let new_tmp_table = {
                        let tmp = self.tables[table_index].lock();
                        if tmp.is_ok() {
                            tmp.unwrap().clone()
                        } else {
                            return Err(Error::other("Couldn't lock table!"));
                        }
                    };
                    let decoded_column_list: Vec<String> = decode_column_list(command_lvl2.0.clone(), new_tmp_table);
                    let command_lvl5 = strip_leading_word(command_lvl4.1);
                    if command_lvl2.0.as_str().contains("*") && binding.split_whitespace().count() == 4 {
                        let search = self.select_all_rows(table_index)?;
                        let table = self.tables[table_index].lock();
                            if table.is_ok() {
                                let ok_table = table.unwrap();
                                let answ = ok_table.select_data(decoded_column_list, search);
                                // Successful decoding of syntax!
                                if self.job_history {
                                    let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                    if history_table.is_ok() {
                                        let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                    }
                                }
                                return Ok(answ); 
                            } else {
                                return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                            }
                    } else if !binding.contains("where") {
                        let search = self.select_all_rows(table_index)?;
                        let table = self.tables[table_index].lock();
                            if table.is_ok() {
                                let ok_table = table.unwrap();
                                let answ = ok_table.select_data(decoded_column_list, search);
                                if self.job_history {
                                    let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                    if history_table.is_ok() {
                                        let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                    }
                                }
                                return Ok(answ);
                            } else {
                                return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                            }
                    } else if command_lvl5.0.as_str().contains("where") {
                        let conditions = command_lvl5.1;
                        let search = self.search_conditionals(conditions.clone(), table_index)?;
                        let table = self.tables[table_index].lock();
                            if table.is_ok() {
                                let ok_table = table.unwrap();
                                let answ = ok_table.select_data(decoded_column_list, search);
                                // Successful decoding of syntax!
                                if self.job_history {
                                    let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                    if history_table.is_ok() {
                                        let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                    }
                                }
                                return Ok(answ);                           
                            } else {
                                return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                            }
                    } else {
                        return Err(Error::other(format!("Invalid nql syntax. {:?} should be 'where'", command_lvl5.1)));
                    }
                } else {
                    return Err(Error::other(format!("Invalid nql syntax. {:?} should be 'from'", command_lvl3.1)));
                }
            },
            "get" => {
                let command_lvl2 = strip_leading_word(command_lvl1.1);
                match command_lvl2.0.as_str() {
                    "min" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1);
                        if command_lvl3.0.contains("in") {
                            let command_lvl4 = strip_leading_word(command_lvl3.1);
                            let columnname = command_lvl4.0;
                            let command_lvl5 = strip_leading_word(command_lvl4.1);
                            if command_lvl5.0.as_str().contains("from") {
                                let tablename = command_lvl5.1;
                                let table_index = self.search_for_table(tablename)?;
                                let table = self.tables[table_index].lock();
                                if table.is_ok() {
                                    let ok_table = table.unwrap();
                                    let column_index = ok_table.search_for_column(columnname)?;
                                    let answ = ok_table.columns[column_index].min();
                                    // Successful decoding of syntax!
                                    if self.job_history {
                                        let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                        if history_table.is_ok() {
                                            let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                        }
                                    }
                                    return Ok(answ);
                                }  else {
                                    return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                                }
                            } else {
                                return Err(Error::other(format!("Invalid nql syntax. {:?} should be one 'from'", command_lvl5.0)));
                            }
                        } else {
                            return Err(Error::other(format!("Invalid nql syntax. {:?} should be one 'in'", command_lvl3.0)));
                        }
                        
                    },
                    "max" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1);
                        if command_lvl3.0.contains("in") {
                            let command_lvl4 = strip_leading_word(command_lvl3.1);
                            let columnname = command_lvl4.0;
                            let command_lvl5 = strip_leading_word(command_lvl4.1);
                            if command_lvl5.0.as_str().contains("from") {
                                let tablename = command_lvl5.1;
                                let table_index = self.search_for_table(tablename)?;
                                let table = self.tables[table_index].lock();
                                if table.is_ok() {
                                    let ok_table = table.unwrap();
                                    let column_index = ok_table.search_for_column(columnname)?;
                                    let answ = ok_table.columns[column_index].max();
                                    // Successful decoding of syntax!
                                    if self.job_history {
                                        let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                        if history_table.is_ok() {
                                            let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                        }
                                    }
                                    return Ok(answ);
                                } else {
                                    return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                                }
                            } else {
                                return Err(Error::other(format!("Invalid nql syntax. {:?} should be one 'from'", command_lvl5.0)));
                            }
                        } else {
                            return Err(Error::other(format!("Invalid nql syntax. {:?} should be one 'in'", command_lvl3.0)));
                        }
                    },
                    "len" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1);
                        if command_lvl3.0.contains("of") {
                            let command_lvl4 = strip_leading_word(command_lvl3.1);
                            let tablename = command_lvl4.0;
                            let table_index = self.search_for_table(tablename)?;
                            let table = self.tables[table_index].lock();
                            if table.is_ok() {
                                let ok_table = table.unwrap();
                                let answ = ok_table.len();
                                if self.job_history {
                                    let history_table = self.tables[self.job_history_table_index.unwrap()].lock();
                                    if history_table.is_ok() {
                                        let _ = self.write_history(binding, date, start, history_table.unwrap())?;
                                    }
                                }
                                return Ok(Success::Result(vec![Data::from(answ.to_string(), self.split_pattern.clone())]));
                            } else {
                                return Err(Error::other("Couldn't lock Table! Aborting task, no data changed!"));
                            }
                        } else {
                            return Err(Error::other(format!("Invalid nql syntax. {:?} should be one 'of'", command_lvl3.0)));
                        }
                    },
                    _ => {
                            return Err(Error::other(format!("Invalid nql syntax. {:?} should be one of [min/max/len]", command_lvl2.0)));
                        },
                    }
                },
            _ => { 
                println!("ERROR: {:?} | {:?} | {:?}", query, command_lvl1.0, command_lvl1.1);
                return Err(Error::other("Invalid nql syntax."));
            },
        }
    }
    fn write_history(&self, binding: String, date: String, start: Instant, table: MutexGuard<Table>) -> Result<(), Error> {
        // I use length => no need to add +1, len does that by
        // itself.
        let mut ok_table = table;
        let id = ok_table.len().to_string();
        let duration = start.elapsed().as_micros().to_string();
        let sp = &self.split_pattern;
        let decoded = decode_list_columndata(format!("(id = {id}{sp} command = {binding}{sp} time = {date}{sp} duration = {duration})"), self.split_pattern.clone());
        let _ = ok_table.new_data(decoded);
        return Ok(());
        
    }
    /// Check if a table exists. returns `true` if it is found, `false` otherwise.
    pub fn exists_table(&self, name: String) -> Result<bool, Error> {
        for table in &self.tables {
            let temp = table.lock();
            if temp.is_ok() {
                let ok_store = temp.unwrap();
                if ok_store.name == name {
                    return Ok(true);
                }
            } else {
                return Err(Error::other("Couldn't lock table!"));
            }
        }
        return Ok(false);
    }
    fn select_all_rows(&self, table_index: usize) -> Result<Vec<usize>, Error> {
        let table = self.tables[table_index].lock();
        if table.is_ok() {
            let ok_table = table.unwrap();
            let out = ok_table.select_all_rows();
            return Ok(out.to_vec());
        } else {
            return Err(Error::other("Couldn't lock table!"));
        }
    }
    fn search_conditionals(&self, conditions: String, table_index: usize) -> Result<Vec<usize>, Error> {
        let decoded_conditions = decode_list_conditions(conditions, self.split_pattern.clone())?;
        let mut encoded_conditions = encode_list_conditions(decoded_conditions, self.split_pattern.clone())?;
        let mut found_data: Vec<usize> = Vec::new();

        // Len can only be:
        // 0 = error, 
        // 1 == one set of 'columname = data', 
        // 3 == 'columname = data' 'and/not/or' 'other_columnname = other_data'
        // >= 3 == more: 'and' 'name = data'
        if encoded_conditions.len() == 0 {
            return Err(Error::other("Invalid nql syntax."));
        } else if encoded_conditions.len() == 1 {
            let data_query = encoded_conditions.first();
            if data_query.is_some() {
                let name = &data_query.unwrap().0;
                let data = &data_query.unwrap().1;
                let table = self.tables[table_index].lock();
                if table.is_ok() {
                    let ok_table = table.unwrap();
                    let search = ok_table.search_column_data(name.to_string(), data.clone())?;
                    // as there is no other elements, no need for push, just set:
                    found_data = search;
                } else {
                    return Err(Error::other("Couldn't lock table!"));
                }
            } else {
                return Err(Error::other(format!("Invalid nql syntax: {:?} = should be a column name and data", encoded_conditions[0])));
            }
        } else if encoded_conditions.len() == 3 {
            // fn tbd(input: Vec<(String, Data)>) -> Vec<(usize, Data)>
            let data_query = &encoded_conditions[0];
            let name = &data_query.0;
            let data = &data_query.1;
            let condition = &encoded_conditions[1].0;
            let other_query = &encoded_conditions[2];
            let other_name = &other_query.0;
            let other_data = &other_query.1;
            let table = self.tables[table_index].lock();
            if table.is_ok() {
                let ok_table = table.unwrap();
                let search = ok_table.search_column_data(name.to_string(), data.clone())?;
                let other_search = ok_table.search_column_data(other_name.to_string(), other_data.clone())?;
                let condition_check = condition_check(search, condition.to_string(), other_search)?;
                // as there is no other elements, no need for push, just set:
                found_data = condition_check;
            } else {
                return Err(Error::other("Couldn't lock table!"));
            }
        } else if encoded_conditions.len() > 3 {
            // fn tbd(input: Vec<(String, Data)>) -> Vec<(usize, Data)>
            let data_query = encoded_conditions.remove(0);
            let name = data_query.0;
            let data = data_query.1;
            // As I remove from the vec, 1 is now 0
            let condition = encoded_conditions.remove(0).0;
            // As I remove from the vec, 2 is now 0
            let other_query = encoded_conditions.remove(0);
            let other_name = other_query.0;
            let other_data = other_query.1;
            let table = self.tables[table_index].lock();
            if table.is_ok() {
                let ok_table = table.unwrap();
                let search = ok_table.search_column_data(name, data)?;
                let other_search = ok_table.search_column_data(other_name, other_data)?;
                let mut temp_hit_files: Vec<usize>;
                temp_hit_files = condition_check(search, condition, other_search)?;
                let mut read_condition = String::new();
                for entry in encoded_conditions {
                    if entry.1 == Data::default() {
                        // Has to be a conditional
                        read_condition = entry.0;
                    } else {
                        let diff_name = entry.0;
                        let diff_data = entry.1;
                        let diff_search = ok_table.search_column_data(diff_name, diff_data)?;
                        temp_hit_files = condition_check(temp_hit_files, read_condition.clone(), diff_search)?;
                    }
                }
                // As the code above ended up exhaustive, again just set:
                found_data = temp_hit_files;
            } else {
                return Err(Error::other("Couldn't lock table!"));
            }
        }
        return Ok(found_data);
    }
    fn search_for_table(&self, tablename: String) -> Result<usize, Error> {
        let mut counter: usize = 0;
        for entry in &self.tables {
            let table = entry.lock();
            if table.is_ok() {
                let ok_table = table.unwrap();
                if ok_table.name.eq(&tablename) {
                    return Ok(counter);
                }
            }
            counter += 1;
        }
        return Err(Error::other(format!("Table with name {} not found.", tablename)));
    }
    fn delete_table(&mut self, tablename: String) -> Result<Success, Error> {
        let _ = self.tables.remove(self.search_for_table(tablename)?);
        return Ok(Success::SuccessMessage(true))
    }
    fn delete_column(&mut self, tablename: String, columnname: String) -> Result<Success, Error> {
        let table_index = self.search_for_table(tablename)?;
        let table = self.tables[table_index].lock();
        if table.is_ok() {
            let mut ok_table = table.unwrap();
            let answ = ok_table.delete_column(columnname)?;
            if answ == Success::SuccessMessage(true) {
                return Ok(Success::SuccessMessage(true));
            } else {
                return Err(Error::other("Deletion error!"));
            }
        } else {
            return Err(Error::other("Couldn't lock table!"));
        }
    }
}

