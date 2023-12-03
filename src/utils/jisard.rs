// This is the Json-Wizard or Jisard for short.

use json::*;
use std::{io::Read, fs::File, path::Path};

pub fn read_json_from_neithdb_file<P>(filename: P) -> JsonValue where P: AsRef<Path> {
    let mut input = File::open(filename).expect("Unable to open file!");
    let mut buffer = String::new();
    let _ = input.read_to_string(&mut buffer);
    let out = parse(&buffer).expect("Invalid json file!");
    return out;
}

fn write_neithdb_file() {
    unimplemented!()
}
