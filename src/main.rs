use jisard::read_json_from_neithdb_file;

use std::{io::Read, fs::File, path::{Path, PathBuf}};

// This is the Json-Wizard or Jisard for short.
mod jisard;
// The column representation
mod column;
// The table representation
mod table;
// The general data representation
mod data;

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
    pub fn mk_table(&mut self, table_name: String) {
        let new_table = Table::new(table_name);
        self.tables.push(new_table);
    }
    pub fn mk_column() {
        unimplemented!()
    }
    pub fn update() {
        unimplemented!()
    }
    pub fn delete() {
        unimplemented!()
    }
    pub fn select() {
        unimplemented!()
    }
    pub fn max() {
        unimplemented!()
    }
    pub fn min() {
        unimplemented!()
    }
    pub fn len() {
        unimplemented!()
    }
}
// Add my own file extension, because I can! By first removing any the user might have set,
// and then adding on my own.
fn canonize_path(value: PathBuf) -> PathBuf {
        let mut path = value;
        path.set_extension("");
        path.set_extension("neithdb");
        return path;
}
fn check_for_persistant_db(filename: PathBuf) -> bool {
    match filename.try_exists() {
        Ok(result) => return result,
        _ => return false,
    }
}
// I guess I just need this for debugging, as it really makes no sense in the finished library.
fn main() {
    let test = Neith::connect("test.neithdb");
    println!("NeitData: {:?}", test);
}

#[cfg(test)]
mod tests {
    use crate::Neith;

    #[test]
    fn read_neithdb_file() {
        let test = Neith::connect("test.neithdb");
        let test_json = Neith::connect("test.json");
        assert_eq!(test, test_json);
    }
}
