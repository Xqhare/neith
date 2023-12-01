use std::io::Error;

use json::JsonValue;

#[derive(Clone, Debug)]
pub struct Column {
    name: String,
    unique: bool,
    contents: ColumnData,
}

#[derive(Clone, Debug)]
pub struct ColumnData {
    all_row_data: Vec<Data>,
}

#[derive(Clone, Debug)]
pub enum Data {
    List(Vec<Data>),
    Float(f64),
    Bool(bool),
    String(String),
}

impl Default for ColumnData {
    fn default() -> Self {
        let aaa: Vec<Data> = Vec::new();
        return ColumnData{all_row_data: aaa};
    }
}

impl Data {
    fn from_json_value(value: &JsonValue) -> Result<Self, std::io::Error> {
        if value.is_boolean() {
            let out = value.as_bool();
            if out.is_none() {
                return Err(Error::other(format!("Boolean value is null! Value: {:?}", value)));
            } else {
                return Ok(Self::Bool(out.unwrap()));
            }
        }
        if value.is_number() {
            let out = value.as_number();
            if out.is_none() {
                return Err(Error::other(format!("Float value is null! Value: {:?}", value)));
            } else {
                return Ok(Self::Float(Into::<f64>::into(out.unwrap())));
            }
        }
        if value.is_string() {
            let out = value.clone().take_string();
            if out.is_none() {
                return Err(Error::other(format!("String value is null! Value: {:?}", value)));
            } else {
                return Ok(Self::String(out.unwrap()));
            }
        }
        return Err(Error::other("Failure to read json value"));
    }
    /// Makes a new empty list!
    fn new_list() -> Self {
        let out: Vec<Data> = Vec::new();
        return Self::List(out);
    }
    fn make_list(json_array: &JsonValue) -> Self {
        let mut out: Vec<Data> = Vec::new();
        for entry in json_array.members() {
            let decoded = Data::from_json_value(entry).unwrap();
            out.push(decoded);
        }
        return Self::List(out);
    }
}

/* impl Default for Column {
    fn default() -> Self {
        
    }
} */

impl Column {
    pub fn from_neith_json_column(column_value: (&str, &JsonValue)) -> Self {
        let name = column_value.0.to_string();
        println!("NAME: {:?}", name);
        let mut data_object = column_value.1.clone();
        let unique = data_object["unique"].as_bool().expect("Boolean not a boolean!");
        println!("BOOL: {:?}", unique);
        let entry_list = data_object["entry"].members();
        let mut all_row_data: Vec<Data> = Vec::new();
        for thing in entry_list {
            // If entry is an array it is a list!
            if thing.is_array() {
                let list = Data::make_list(thing);
                all_row_data.push(list);
            } else {
                let out = Data::from_json_value(thing).unwrap();
                all_row_data.push(out);
            }
        }
        println!("ROWDATA: {:?}", all_row_data);
        return Column{name, unique, contents: ColumnData { all_row_data}};
    }
}
