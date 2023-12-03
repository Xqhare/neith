use jisard::read_json_from_neithdb_file;

use std::{io::{self, Error}, path::{Path, PathBuf}};

use crate::utils::util::{*};

#[cfg(test)]
mod tests;

// This is the Json-Wizard or Jisard for short.
mod jisard;
// The column representation
mod column;
// The table representation
mod table;
// The general data representation
mod data;
// My util module.
mod utils;

use crate::table::Table;

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
                        println!("cmd4: {:?}", command_lvl4);
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
                        let command_lvl3 = strip_leading_word(command_lvl2.1);
                        if command_lvl3.0.as_str() == "with" {
                        } else {
                            return Err(Error::other("Invalid nql syntax."));
                        }
                    },
                    "data" => {},
                    _ => return Err(Error::other("Invalid nql syntax.")),
                }
                return Ok(true);
            },
            "delete" => {
                /* match command_level2.0.as_str() {
                    "table" => {},
                    "column" => {},
                    "data" => {},
                    _ => return Err(Error::other("Invalid nql syntax.")),
                } */
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
    
}


fn main() {
    let con = Neith::connect("test.neithdb");
    let new_table = con.execute("new table testtable with (column1 true, column2 false, column3 false)");
    println!("{:?}", new_table)
}
