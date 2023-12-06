
#![feature(iter_intersperse)]
use std::{io::{self, Error}, path::{Path, PathBuf}};

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
use success::Success;
use utils::{util, jisard::write_neithdb_file};

#[derive(Clone, Debug, PartialEq)]
pub struct Neith {
    path: PathBuf,
    ram_mode: bool,
    job_history: bool,
    tables: Vec<Table>,
}

impl Default for Neith {
    fn default() -> Self {
        let tables: Vec<Table> = Vec::new();
        let ram_mode = true;
        let job_history = false;
        let path = PathBuf::new();
        return Neith{ tables, path, ram_mode, job_history,};
    }
    
}

impl From<PathBuf> for Neith {
    fn from(value: PathBuf) -> Self {
        let path = canonize_path(value);
        let read_file = read_json_from_neithdb_file(path.clone());
        let mut tables: Vec<Table> = Vec::new();
        let ram_mode = false;
        let job_history = false;
        for table in read_file.entries() {
            let table = Table::from(table);
            tables.push(table);
        }
        return Neith{ tables, path, ram_mode, job_history,};
    }
}

impl Neith {
    /// Creates a new Neith instance, with no contents.
    /// For general use, `connect(filename)` is highly recommended.
    pub fn new(value: PathBuf, ram_mode: bool, job_history: bool) -> Self {
        let path = canonize_path(value);
        let tables: Vec<Table> = Vec::new();
        return Neith{ tables, path, ram_mode, job_history,};
    }
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
    // TODO: LEAVE THIS LAST. Its really only a flag put on top of everything, as Neith loads
    // everything into memory anyway, so I just need to not save the data below.
    // WIP Ram only mode -> no saving, all data is lost on shutdown!
    pub fn connect_ram_mode(job_history: bool) -> Self {
        let mut connection = Neith::default();
        let _ = connection.set_job_history(job_history);
        return connection;
    }
    pub fn set_job_history(&mut self, value: bool) -> Success {
        self.job_history = value;
        return Success::SuccessMessage(value);
    }
    pub fn save(&mut self) -> Success {
        unimplemented!()
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
    pub fn execute(&mut self, query: &str) -> Result<Success, io::Error> {
        let binding = Into::<String>::into(query);
        let command_lvl1 = strip_leading_word(binding);
        match command_lvl1.0.as_str() {
            "new" => {
                let command_lvl2 = strip_leading_word(command_lvl1.1);
                let command_lvl3 = strip_leading_word(command_lvl2.1.clone());
                let tablename =  command_lvl3.0.clone();
                match command_lvl2.0.as_str() {
                    "table" => {
                        let command_lvl4 = strip_leading_word(command_lvl3.1.clone());
                        if command_lvl4.0.as_str().contains("with") {
                            let columns = decode_columnmaker(command_lvl4.1).unwrap();
                            let answ = Table::from((tablename, columns));
                            self.tables.push(answ);
                            // Successful decoding of syntax!
                            return Ok(Success::SuccessMessage(true));
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "column" => {
                        let command_lvl4 = strip_leading_word(command_lvl3.1.clone());
                        if command_lvl4.0.as_str().contains("with") {
                            let columns = decode_columnmaker(command_lvl4.1).unwrap();
                            let table_index = self.search_for_table(tablename)?;
                            let answ = self.tables[table_index].new_columns(columns);
                            // Successful decoding of syntax!
                            if answ == Success::SuccessMessage(true) {
                                return Ok(answ);
                            } else {
                                return Err(Error::other("Invalid nql syntax."));
                            }
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "data" => {
                        let command_lvl4 = command_lvl3.1.clone();
                        let decoded = decode_list_columndata(command_lvl4).unwrap();
                        let table_index = self.search_for_table(tablename)?;
                        let answ = self.tables[table_index].new_data(decoded)?;
                        if answ == Success::SuccessMessage(true) {
                            return Ok(answ);
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }

                    },
                    _ => return Err(Error::other("Invalid nql syntax.")),
                }
            },
            "delete" => {
                let command_lvl2 = strip_leading_word(command_lvl1.1);
                match command_lvl2.0.as_str() {
                    "table" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1.clone());
                        if command_lvl3.0.as_str().contains("with") {
                            let tablename = command_lvl3.1;
                            let answ = self.delete_table(tablename);
                            if answ.is_ok() {
                                return Ok(answ.unwrap());
                            } else {
                                return Err(Error::other("Invalid nql syntax."));
                            }
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "column" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1.clone());
                        if command_lvl3.0.as_str().contains("with") {
                            let command_lvl4 = strip_leading_word(command_lvl3.1);
                            let columnname = command_lvl4.0;
                            let command_lvl5 = strip_leading_word(command_lvl4.1);
                            if command_lvl5.0.as_str().contains("in") {
                                let tablename = command_lvl5.1;
                                let answ = self.delete_column(tablename, columnname);
                                if answ.is_ok() {
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
                        let command_lvl3 = strip_leading_word(command_lvl2.1.clone());
                        if command_lvl3.0.as_str().contains("in") {
                            let command_lvl4 = strip_leading_word(command_lvl3.1);
                            let tablename = command_lvl4.0;
                            let table_index = self.search_for_table(tablename)?;
                            let command_lvl5 = strip_leading_word(command_lvl4.1);
                            if command_lvl5.0.as_str().contains("where"){
                                let conditions = command_lvl5.1;
                                let finished_search = self.search_conditionals(conditions, table_index)?;
                                let answ = self.tables[table_index].delete_data(finished_search);
                                if answ.is_ok() {
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
                        let decoded_list = decode_list_columndata(command_lvl5.1)?;
                        let table_index = self.search_for_table(tablename)?;
                        let search = self.search_conditionals(conditions, table_index)?;
                        let answ = self.tables[table_index].update_data(decoded_list, search)?;
                        match answ {
                            Success::SuccessMessage(true) => return Ok(answ),
                            _ => return Err(Error::other(format!("Invalid nql syntax."))),
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
                    let decoded_column_list: Vec<String> = decode_column_list(command_lvl2.0, self.tables[table_index].clone());
                    let command_lvl5 = strip_leading_word(command_lvl4.1);
                    if command_lvl5.0.as_str().contains("where") {
                        let conditions = command_lvl5.1;
                        let search = self.search_conditionals(conditions.clone(), table_index)?;
                        let answ = self.tables[table_index].clone().select_data(decoded_column_list.clone(), search.clone());
                        return Ok(answ);
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
                                let column_index = self.tables[table_index].search_for_column(columnname)?;
                                let answ = self.tables[table_index].columns[column_index].min();
                                return Ok(answ);
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
                                let column_index = self.tables[table_index].search_for_column(columnname)?;
                                let answ = self.tables[table_index].columns[column_index].max();
                                return Ok(answ);
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
                            let answ = self.tables[table_index].len();
                            // This is stupid and I love it!
                            let temp_str = answ.to_string();
                            let encoded_data = vec![Data::from(temp_str)];
                            return Ok(Success::Result(encoded_data));
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
    fn search_conditionals(&self, conditions: String, table_index: usize) -> Result<Vec<usize>, Error> {
        let decoded_conditions = decode_list_conditions(conditions)?;
        let mut encoded_conditions = encode_list_conditions(decoded_conditions)?;
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
                let search = self.tables[table_index].search_column_data(name.to_string(), data.clone())?;
                // as there is no other elements, no need for push, just set:
                found_data = search;
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
            let search = self.tables[table_index].search_column_data(name.to_string(), data.clone())?;
            let other_search = self.tables[table_index].search_column_data(other_name.to_string(), other_data.clone())?;
            let condition_check = condition_check(search, condition.to_string(), other_search)?;
            // as there is no other elements, no need for push, just set:
            found_data = condition_check;
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
            let search = self.tables[table_index].search_column_data(name, data)?;
            let other_search = self.tables[table_index].search_column_data(other_name, other_data)?;
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
                    let diff_search = self.tables[table_index].search_column_data(diff_name, diff_data)?;
                    temp_hit_files = condition_check(temp_hit_files, read_condition.clone(), diff_search)?;
                }
            }
            // As the code above ended up exhaustive, again just set:
            found_data = temp_hit_files;
        }
        return Ok(found_data);
    }
    fn search_for_table(&self, tablename: String) -> Result<usize, Error> {
        let mut counter: usize = 0;
        for entry in &self.tables {
            if entry.name.eq(&tablename) {
                return Ok(counter);
            }
            counter += 1;
        }
        return Err(Error::other(format!("Table with name {} not found.", tablename)));
    }
    fn delete_table(&mut self, tablename: String) -> Result<Success, Error> {
        let table_index = self.search_for_table(tablename)?;
        let _ = self.tables.remove(table_index);
        return Ok(Success::SuccessMessage(true))
    }
    fn delete_column(&mut self, tablename: String, columnname: String) -> Result<Success, Error> {
        let table_index = self.search_for_table(tablename)?;
        let answ = self.tables[table_index].delete_column(columnname)?;
        if answ == Success::SuccessMessage(true) {
            return Ok(Success::SuccessMessage(true));
        } else {
            return Err(Error::other("Deletion error!"));
        }
    }
}


fn main() {
    let mut con = Neith::connect("test.neithdb");
    let job_history = con.set_job_history(true);
    let new_table = con.execute("new table testtable with (column1 true, column2 false, column3 false)");
    let new_columns = con.execute("new column testtable with (column4 false, column5 false, column6 false, column7 false)");
    let _ = con.execute("new column testtable with (column8 false)");
    let new_data_column1 = con.execute("new data testtable (column1 = 1, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text)), column6 = this will be deleted!");
    let new_data_column2 = con.execute("new data testtable (column1 = 2, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text))");
    // let _ = con.execute("delete table with table_name0");
    // let _ = con.execute("delete column with column6 in testtable");
    // let _ = con.execute("delete data in testtable where [column1 = 2, and column3 = true]"); 
    let newdata = con.execute("new data testtable (column1 = 3, column2 = 1, column3 = false, column4 = some, column7 = this will not be deleted!)");
    let upd = con.execute("update testtable where [column1 = 3] with (column7 = this was updated!)");
    let answ = con.execute("select * from testtable where [column1 = 1, or column1 = 3]");
    let answ2 = con.execute("select (column7, column1) from testtable where [column1 = 3]");
    let min = con.execute("get min in column1 from testtable");
    let min2 = con.execute("get min in column7 from testtable");
    let min3 = con.execute("get min in column5 from testtable");
    let max = con.execute("get max in column1 from testtable");
    let len = con.execute("get len of testtable");
    /* println!("==========");
    println!("{:?}", answ.unwrap());
    println!("{:?}", answ2.unwrap());
    println!("==========");
    println!("MIN COL1 = {:?}", min);
    println!("MIN COL7 = {:?}", min2);
    println!("MIN COL5 = {:?}", min3);
    println!("MAX COL1 = {:?}", max);
    println!("LEN = {:?}", len);
    println!("{:?} | {:?}", new_table, new_columns);
    for table in con.tables.clone() {
        println!("TABLE: {:?}", table.name);
        for column in table.columns {
            println!("{:?}", column);
        }
    }
    println!("---");
    println!("{:?}", con.tables);
    println!("---");
    println!("{:?} | {:?}", new_data_column1, new_data_column2) */
    write_neithdb_file(con);
}
