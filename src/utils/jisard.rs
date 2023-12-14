//! This is the Json-Wizard or Jisard for short.
//! It contains the logic for reading and writing a json file.

use json::*;
use std::{io::Read, fs::{File, self}, path::Path};

use crate::{Neith, data::Data, success::Success};

/// Takes a path and reads the json file at the location the path points to.
///
/// ## Returns
/// Returns a `JsonValue`.
pub fn read_json_from_neithdb_file<P>(filename: P) -> JsonValue where P: AsRef<Path> {
    let mut input = File::open(filename).expect("Unable to open file!");
    let mut buffer = String::new();
    let _ = input.read_to_string(&mut buffer);
    let out = parse(&buffer).expect("Invalid json file!");
    return out;
}
/// Takes the database and writes it to file.
///
/// ## Returns
/// A generic Success message.
///
/// ## Errors
/// Can `JsonError` during json encoding or saving to disc.
pub fn write_neithdb_file(neith: Neith) -> Result<Success> {
    let mut json_tables = JsonValue::new_object();
    for table in &neith.tables {
        let tablename = &table.name;
        let mut json_table = JsonValue::new_object();
        for column in &table.columns {
            let columnname = &column.name;
            let unique = column.unique;
            let mut data_array = JsonValue::new_array();
            for data in &column.contents.all_row_data {
                let _answ = data_array.push(decode_data_to_jsonval(data.clone()))?;
            }
            let mut json_column = JsonValue::new_object();
            let _answ0 = json_column.insert("unique", JsonValue::Boolean(unique))?;
            let _answ1 = json_column.insert("entry", data_array)?;
            let _answ3 = json_table.insert(&columnname, json_column)?;
        }
        let _answ2 = json_tables.insert(&tablename, json_table)?;
    }
    let file = fs::File::create(neith.path);
    let fin = json_tables.write(&mut file.unwrap());
    if fin.is_ok() {
        return Ok(Success::SuccessMessage(true));
    } else {
        return Err(JsonError::wrong_type("Error during writing!"));
    }
}
/// Takes in a `neith::Data` and encodes it as a `JsonValue`.
/// This supports up to 5 nested lists!
///
/// ## Returns
/// A `JsonValue` containing the passed in `Data`.
fn decode_data_to_jsonval(neith_data: crate::Data) -> JsonValue {
    // Nested functions, now in a database near you!
    /// Converts the data to a `JsonValue`. Should a list be passed in, its set to `Null.`
    ///
    /// ## Returns
    /// A `JsonValue`.
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
                                                    let _ = array4.push(make_json_val(thing5));
                                                }
                                                let _ = array3.push(array4);
                                            }
                                            let _ = array3.push(make_json_val(thing4));
                                        }
                                        let _ = array2.push(array3);
                                    }
                                    let _ = array2.push(make_json_val(thing3));
                                }
                                let _ = array1.push(array2);
                            }
                            let _ = array1.push(make_json_val(thing2));
                        }
                        let _ = array0.push(array1);
                    }
                    let _ = array0.push(make_json_val(thing));
                }
                let _ = array.push(array0);
            }
            let _ = array.push(make_json_val(data));
        }
        return array;
    }
    return make_json_val(neith_data);
}
