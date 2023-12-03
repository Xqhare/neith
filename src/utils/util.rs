use std::{path::PathBuf, io::Error};

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
