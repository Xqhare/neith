use std::{io::Error, str::FromStr};

use json::JsonValue;


#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    List(Vec<Data>),
    Float(f64),
    Bool(bool),
    String(String),
    Null(),
}

impl Default for Data {
    fn default() -> Self {
        return Data::Null();
    }
}

impl From<String> for Data {
    fn from(value: String) -> Self {
        let bool_test = value.parse::<bool>();
        if bool_test.is_ok() {
            return Data::Bool(bool_test.unwrap());
        }
        let float_test = value.parse::<f64>();
        if float_test.is_ok() {
            return Data::Float(float_test.unwrap());
        }
        // (1, 10.1, true, test)
        if value.starts_with("(") && value.ends_with(")") {
            println!("LIST DETECTED");
            let temp_val = value.replace("(", "").replace(")", "");
            let split = temp_val.split(",");
            let mut out: Vec<Data> = Vec::new();
            for entry in split {
                // Will this recursive call work?
                let data = self::Data::from_single_for_list(entry.to_string());
                out.push(data);
            }
            return Data::List(out);
        } else {
            return Data::String(value);
        }

    }
}

impl Data {
    fn from_single_for_list(value: String) -> Self {
        let bool_test = value.parse::<bool>();
        if bool_test.is_ok() {
            return Data::Bool(bool_test.unwrap());
        }
        let float_test = value.parse::<f64>();
        if float_test.is_ok() {
            return Data::Float(float_test.unwrap());
        } else {
            return Data::String(value);
        }
    }
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


