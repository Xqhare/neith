// This is the Json-Wizard or Jisard for short.

use json::*;
use std::{io::{Read, Write, Error, self}, fs::{File, self}, path::Path};

use crate::{Neith, data::{Data, self}, success::Success};

pub fn read_json_from_neithdb_file<P>(filename: P) -> JsonValue where P: AsRef<Path> {
    let mut input = File::open(filename).expect("Unable to open file!");
    let mut buffer = String::new();
    let _ = input.read_to_string(&mut buffer);
    let out = parse(&buffer).expect("Invalid json file!");
    println!("{:?}", out);
    println!("=============================");
    println!("=============================");
    println!("=============================");
    return out;
}

pub fn write_neithdb_file(neith: Neith) -> Result<Success> {
    println!("START:: {:?}", neith.clone());
    let mut json_tables = JsonValue::new_object();
    for table in neith.tables {
        let tablename = table.name;
        println!("{:?}", tablename);
        let mut json_columns = JsonValue::new_object();
        for column in table.columns {
            let columnname = column.name;
            let unique = column.unique;
            println!("{:?}", columnname);
            println!("{:?}", unique);
            let mut data_array = JsonValue::new_array();
            for data in column.contents.all_row_data {
                let _answ = data_array.push(decode_data_to_jsonval(data))?;
            }
            let _answ0 = json_columns.insert("unique", JsonValue::Boolean(unique))?;
            let _answ1 = json_columns.insert(&columnname, data_array)?;
        }
        let _answ2 = json_tables.insert(&tablename, json_columns)?;
    }
    let file = fs::File::create(neith.path);
    let _fin = json_tables.write(&mut file.unwrap()).unwrap();
    return Ok(Success::SuccessMessage(true));
}
fn decode_data_to_jsonval(neith_data: crate::Data) -> JsonValue {
    // Nested functions, now in a database near you!
    fn make_json_val(data: Data) -> JsonValue {
        match data.get_type().as_str() {
            "String" => {
                let inner = data.get_string().unwrap();
                return json::JsonValue::String(inner);
            },
            "Bool" => {
                let inner = data.get_bool().unwrap();
                return JsonValue::Boolean(inner);
            },
            "Float" => {
                let inner = data.get_float().unwrap();
                return JsonValue::Number(inner.into());
            },
            // If this does get a list, just Null it? Horrible Idea I think, but what could
            // possibly go wrong? -> This is an apology to future Xqhare who for some reason can't
            // seem to find why his lists are nulled!
            _ => {
                return JsonValue::Null;
            },
        }
    }
    // This monstrosity checks up to 5 nested lists!
    // -> I can't make a recursive function; not one that does what I need it to. Recursive functions are possible, at least according to my googleing, but I can't seem to make them work.
    if neith_data.is_list() {
        let mut array = JsonValue::new_array();
        for data in neith_data.get_list().unwrap() {
            if data.is_list() {
                let mut array0 = JsonValue::new_array();
                for thing in data.get_list().unwrap() {
                    if thing.is_list() {
                        let mut array1 = JsonValue::new_array();
                        for thing2 in thing.get_list().unwrap() {
                            if thing2.is_list() {
                                let mut array2 = JsonValue::new_array();
                                for thing3 in thing2.get_list().unwrap() {
                                    if thing3.is_list() {
                                        let mut array3 = JsonValue::new_array();
                                        for thing4 in thing3.get_list().unwrap() {
                                            if thing4.is_list() {
                                                let mut array4 = JsonValue::new_array();
                                                for thing5 in thing4.get_list().unwrap() {
                                                    array4.push(make_json_val(thing5));
                                                }
                                                array3.push(array4);
                                            }
                                            array3.push(make_json_val(thing4));
                                        }
                                        array2.push(array3);
                                    }
                                    array2.push(make_json_val(thing3));
                                }
                                array1.push(array2);
                            }
                            array1.push(make_json_val(thing2));
                        }
                        array0.push(array1);
                    }
                    array0.push(make_json_val(thing));
                }
                array.push(array0);
            }
            array.push(make_json_val(data));
        }
        return array;
    }
    return make_json_val(neith_data);
}
