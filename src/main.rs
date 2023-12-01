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
    pub tables: Vec<Table>,
}

impl Default for Neith {
    fn default() -> Self {
        let tables: Vec<Table> = Vec::new();
        let path = PathBuf::new();
        return Neith{ tables, path, };
    }
    
}

impl From<PathBuf> for Neith {
    fn from(value: PathBuf) -> Self {
        let mut path = value;
        path.set_extension("");
        path.set_extension("neithdb");
        let read_file = read_json_from_neithdb_file(path.clone());
        let mut tables: Vec<Table> = Vec::new();
        for table in read_file.entries() {
            let table = Table::from_neithdb_table_data(table);
            tables.push(table);
        }
        return Neith{ tables, path};
    }
}

impl Neith {
    /// Creates a new Neith instance, with no contents.
    /// For general use, `connect(filename)` is highly recommended.
    pub fn new(value: PathBuf) -> Self {
        let mut path = value;
        path.set_extension("");
        path.set_extension("neithdb");
        let tables: Vec<Table> = Vec::new();
        return Neith{ tables, path, };
    }
    pub fn connect<P>(filename: P) -> Self where P: AsRef<Path> + Clone, PathBuf: From<P> {
        let mut path: PathBuf = filename.clone().into();
        // Add my own file extension, because I can! By first removing any the user might have set,
        // and then adding on my own.
        path.set_extension("");
        path.set_extension("neithdb");
        if check_for_persistant_db(path.clone()) {
            let connection = Neith::from(path);
            return connection;
        } else {
            let connection = Neith::new(path);
            return connection;
        }
    }
    // TODO: LEAVE THIS LAST. Its really only a flag put on top of everything, as Neith loads
    // everything into memory anyway, so I just need to not save the data below.
    // WIP Ram only mode -> no saving, all data is lost on shutdown!
    pub fn connect_ram_mode<P>(_connection_name: P) -> Self where P: AsRef<Path> + Clone, PathBuf: From<P> {
        unimplemented!()
    }
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
