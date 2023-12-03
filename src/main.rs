use std::{io::{self, Error}, path::{Path, PathBuf}};

use crate::utils::util::{*};

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

#[derive(Clone, Debug, PartialEq)]
pub struct Neith {
    pub path: PathBuf,
    pub ram_mode: bool,
    pub tables: Vec<Table>,
}

impl Default for Neith {
    fn default() -> Self {
        let tables: Vec<Table> = Vec::new();
        let ram_mode = true;
        let path = PathBuf::new();
        return Neith{ tables, path, ram_mode, };
    }
    
}

impl From<PathBuf> for Neith {
    fn from(value: PathBuf) -> Self {
        let path = canonize_path(value);
        let read_file = read_json_from_neithdb_file(path.clone());
        let mut tables: Vec<Table> = Vec::new();
        let ram_mode = false;
        for table in read_file.entries() {
            let table = Table::from(table);
            tables.push(table);
        }
        return Neith{ tables, path, ram_mode, };
    }
}

impl Neith {
    /// Creates a new Neith instance, with no contents.
    /// For general use, `connect(filename)` is highly recommended.
    pub fn new(value: PathBuf, ram_mode: bool) -> Self {
        let path = canonize_path(value);
        let tables: Vec<Table> = Vec::new();
        return Neith{ tables, path, ram_mode, };
    }
    pub fn connect<P>(filename: P) -> Self where P: AsRef<Path> + Clone, PathBuf: From<P> {
        let path = canonize_path(filename.into());
        if check_for_persistant_db(path.clone()) {
            let connection = Neith::from(path);
            return connection;
        } else {
            let connection = Neith::new(path, false);
            return connection;
        }
    }
    // TODO: LEAVE THIS LAST. Its really only a flag put on top of everything, as Neith loads
    // everything into memory anyway, so I just need to not save the data below.
    // WIP Ram only mode -> no saving, all data is lost on shutdown!
    pub fn connect_ram_mode<P>(_connection_name: P) -> Self where P: AsRef<Path> + Clone, PathBuf: From<P> {
        let _connection = Neith::default();
        unimplemented!();
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
    pub fn execute(&mut self, query: &str) -> Result<bool, io::Error> {
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
                            return Ok(true);
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
                                return Ok(true);
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
                            return Ok(true);
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
                                return Ok(true);
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
                        } else {
                            let columnname = command_lvl3.1;
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "data" => {
                        let command_lvl3 = strip_leading_word(command_lvl2.1.clone());
                        if command_lvl3.0.as_str().contains("in") {
                        } else {
                            let tablename = command_lvl3.1;
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    _ => return Err(Error::other("Invalid nql syntax.")),
                }
                println!("DELETE: {:?}", query);
                return Ok(true);
            },
            "update" => {
                println!("UPDATE: {:?}", query);
                return Ok(true);
            },
            "select" => {
                println!("SELECT: {:?}", query);
                return Ok(true);
            },
            "get" => {
                println!("GET: {:?}", query);
                return Ok(true);
            },
            _ => { 
                println!("ERROR: {:?} | {:?} | {:?}", query, command_lvl1.0, command_lvl1.1);
                return Err(Error::other("Invalid nql syntax."));
            },
        }
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
}


fn main() {
    let mut con = Neith::connect("test.neithdb");
    let new_table = con.execute("new table testtable with (column1 true, column2 false, column3 false)");
    let new_columns = con.execute("new column testtable with (column4 false, column5 false)");
    let new_data_column1 = con.execute("new data testtable (column1 = 1, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text))");
    let new_data_column2 = con.execute("new data testtable (column1 = 2, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text))");
    println!("{:?} | {:?}", new_table, new_columns);
    println!("{:?}", con.tables);
    println!("---");
    println!("{:?} | {:?}", new_data_column1, new_data_column2)
}
