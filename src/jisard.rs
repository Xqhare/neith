// This is the Json-Wizard or Jisard for short.

use std::{io::Read, fs::File, path::Path};

use json::*;

use crate::table::Table;

#[derive(Clone, Debug)]
pub struct Neith {
    tables: Vec<Table>,
}

impl Neith {
    pub fn from_neithdb_file<P>(filename: P) -> Self where P: AsRef<Path> {
        let read_file = read_json_from_neithdb_file(filename);
        let mut out: Vec<Table> = Vec::new();
        for table in read_file.entries() {
            let table = Table::from_neithdb_table_data(table);
            out.push(table);
        }
        return Neith{tables: out};
    }
}

fn read_json_from_neithdb_file<P>(filename: P) -> JsonValue where P: AsRef<Path> {
    let mut input = File::open(filename).expect("Unable to open file!");
    let mut buffer = String::new();
    let _ = input.read_to_string(&mut buffer);
    let out = parse(&buffer).expect("Invalid json file!");
    return out;
}

fn write_neithdb_file() {
    unimplemented!()
}
