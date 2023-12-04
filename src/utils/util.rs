use std::{path::PathBuf, io::Error, hash::Hash, collections::{HashMap, hash_map::Entry}};

use crate::{data::Data, success::Success};

// write a decoder for every code step 
pub fn strip_leading_word(to_strip: String) -> (String, String) {
    let split_query: Vec<&str> = to_strip.splitn(2, " ").collect::<Vec<_>>();
    let command = split_query.clone().into_iter().take(1).collect::<String>();
    let remainder = split_query.clone().into_iter().skip(1).collect::<String>();
    return (command.clone(), remainder);
}
// Add my own file extension, because I can! By first removing any the user might have set,
// and then adding on my own.
pub fn canonize_path(value: PathBuf) -> PathBuf {
        let mut path = value;
        path.set_extension("");
        path.set_extension("neithdb");
        return path;
}
pub fn check_for_persistant_db(filename: PathBuf) -> bool {
    match filename.try_exists() {
        Ok(result) => return result,
        _ => return false,
    }
}
pub fn decode_columnmaker(input: String) -> Result<Vec<(String, bool)>, Error> {
    // ('columnname' 'unique, ...') is left.
    let no_parenthesis = input.replace("(", "").replace(")", "");
    let column_names = no_parenthesis.split(",");
    let mut temp_column_bind: Vec<(String, bool)> = Vec::new();
    for name in column_names {
        let split_column = name.split_whitespace();
        if split_column.clone().count() != 2 {
            return Err(Error::other(format!("Invalid nql syntax. Wrong amount elements in column creation. {:?} => should contain the name and unique bool, nothing else.", name)));
        } else {
            let name = split_column.clone().take(1).collect::<String>();
            let unique = split_column.clone().skip(1).collect::<String>().parse::<bool>();
            if unique.is_ok() {
                temp_column_bind.push((name, unique.unwrap()));
            } else {
                return Err(Error::other(format!("Invalid nql syntax. Could not parse {} into a boolean!", split_column.skip(1).collect::<String>())));
            }
        }
    }
    return Ok(temp_column_bind);
}
// decode this: (column1 = 2, column2 = -2.04, column3 = true, column4 = text, column5 = (1.04, 2, false, more text))
pub fn decode_list_columndata(list_val: String) -> Result<Vec<(String, Data)>, Error> {
    let mut out: Vec<(String, Data)> = Vec::new();
    let mut clean_in = list_val.replacen("(", "", 1);
    if clean_in.ends_with("))") {
        clean_in = clean_in.replacen("))", ")", 1);
    } else {
        clean_in = clean_in.trim_end_matches(")").to_string();
    }
    let split = clean_in.split(",");
    let mut list_store: String = String::new();
    let mut list_check = false;
    for entry in split {
        if entry.contains("(") {
            list_check = true;
        }
        if list_check {
            println!("LIST DETECTED");
            list_store.push_str(entry);
            if entry.contains(")") {
                list_check = false;
                let new = decode_single_columndata(&list_store)?;
                out.push(new);
            }
            list_store.push_str(",");
        } else {
            println!("DEBUGING == {:?}", entry);
            let new = decode_single_columndata(entry)?;
            out.push(new);
        }
    }
    println!("{:?}", out);
    return Ok(out);
}
// decode this: (other_columnname = newdata) -> as smaller more focused function for more broad
// usage during decoding.
pub fn decode_single_columndata(single_val: &str) -> Result<(String, Data), Error> {
    // I get: columnname = data
    let cleaned_input = single_val.replace("=", "");
    let split_input = cleaned_input.split_whitespace();
    let name = split_input.clone().take(1).collect::<String>();
    let data = Data::from(split_input.clone().skip(1).collect::<String>());
    return Ok((name, data));
}

// decode this: ['columnname' = 'data', {and/not/or} 'other_columnname' = 'other data', ...]
//
pub fn decode_list_conditions(value: String) -> Result<Vec<String>, Error> {
    let split = value.split(",");
    let mut out: Vec<String> = Vec::new();
    for entry in split.clone() {
        if entry.starts_with(" and") || entry.starts_with(" not") || entry.starts_with(" or") {
            let input = entry.trim_start().splitn(2, " ");
            let condition = input.clone().take(1).collect::<String>();
            out.push(condition);
            let pair = input.skip(1).collect::<String>();
            out.push(pair);
        } else if entry.contains(" = ") {
            out.push(entry.to_string());
        } else {
            return Err(Error::other(format!("Invalid nql syntax. Only 'column = data' pairs or conditionals! {:?}", split)));
        }
    }
    return Ok(out);
}

pub fn encode_list_conditions(value: Vec<String>) -> Result<Vec<(String, Data)>, Error> {
    let mut encoding_list: Vec<(String, Data)> = Vec::new();
    for thing in value {
        let mut cleaned_thing = thing.clone();
        if thing.contains("[") {
            let temp = thing.replace("[", "");
            cleaned_thing = temp;
        } else if thing.contains("]") {
            let temp = thing.replace("]", "");
            cleaned_thing = temp;
        }
        if cleaned_thing.contains(" = ") {
            let decode_columndata = decode_single_columndata(&cleaned_thing)?;
            let name = decode_columndata.0;
            let data = decode_columndata.1;
            encoding_list.push((name, data));
        } else if cleaned_thing.contains("and") {
            encoding_list.push((cleaned_thing, Data::default()));
        } else if cleaned_thing.contains("not") {
            encoding_list.push((cleaned_thing, Data::default()));
        } else if cleaned_thing.contains("or") {
            encoding_list.push((cleaned_thing, Data::default()));
        } else {
            return Err(Error::other(format!("Invalid nql syntax. This should be either a conditional or a single_columndata = {}", cleaned_thing)));
        }
    }
    return Ok(encoding_list);
}
pub fn condition_check(search: Vec<usize>, condition: String, other_search: Vec<usize>) -> Result<Vec<usize>, Error> {
    let mut found_data: Vec<usize> = Vec::new();
    match condition.as_str() {
        "and" => {
            for entry in search {
                if other_search.contains(&entry) {
                    found_data.push(entry);
                }
            }
        },
        "not" => {
            for entry in search {
                if !other_search.contains(&entry) {
                    found_data.push(entry);
                }
            }
        },
        "or" => {
            let mut combined_vec: Vec<usize> = Vec::new();
            for entry in search {
                if !combined_vec.contains(&entry) {
                    combined_vec.push(entry);
                }
            }
            for entry in other_search {
                if !combined_vec.contains(&entry) {
                    combined_vec.push(entry);
                }
            }

            
        },
        "xor" => {
            let mut combined_vec: Vec<usize> = Vec::new();
            for entry in search {
                if !combined_vec.contains(&entry) {
                    combined_vec.push(entry);
                }
            }
            for entry in other_search {
                if !combined_vec.contains(&entry) {
                    combined_vec.push(entry);
                } else {
                    let index = combined_vec.iter().position(|n| n == &entry).unwrap();
                    let _ = combined_vec.remove(index);
                }
            }
        },
        _ => return Err(Error::other("Invalid nql syntax.")),
    }
    return Ok(found_data);
}
// WIP
fn thing_in_list(thing: usize, list: Vec<usize>) -> Option<usize> {
    for entry in list {
        if thing == entry {
            return Some(entry);
        }
    }
    return None;
}
