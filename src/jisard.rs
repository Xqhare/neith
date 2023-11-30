// This is the Json-Wizard or Jisard for short.

use std::{io::Read, fs::File, path::Path};

use json::*;

pub fn main() {
    let aaa = read_json_from_file("test.json");
    // This makes tables! 
    for table in aaa.entries() {
        // table.0 == table name
        // table.1 == table contents
        decode_json_table(table.1.clone())
    }
}

fn decode_json_table(table_contents: JsonValue) {
    for entry in table_contents.entries() {
        println!("{:?}", entry);
        println!("------------------");
    }
}

fn read_json_from_file<P>(filename: P) -> JsonValue where P: AsRef<Path> {
    let mut input = File::open(filename).expect("Unable to open file!");
    let mut buffer = String::new();
    let _ = input.read_to_string(&mut buffer);
    let out = parse(&buffer).expect("Invalid json file!");
    return out;
}

