use std::io::Error;

use json::JsonValue;


#[derive(Clone, Debug)]
pub enum Data {
    List(Vec<Data>),
    Float(f64),
    Bool(bool),
    String(String),
}

impl Data {
    pub fn from_json_value(value: &JsonValue) -> Result<Self, std::io::Error> {
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
    pub fn new_list() -> Self {
        let out: Vec<Data> = Vec::new();
        return Self::List(out);
    }
    pub fn make_list(json_array: &JsonValue) -> Self {
        let mut out: Vec<Data> = Vec::new();
        for entry in json_array.members() {
            let decoded = Data::from_json_value(entry).unwrap();
            out.push(decoded);
        }
        return Self::List(out);
    }
}


