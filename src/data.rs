use std::io::Error;

use json::JsonValue;


#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
            let temp_val = value.replace("(", "").replace(")", "");
            let split = temp_val.split(",");
            let mut out: Vec<Data> = Vec::new();
            for entry in split {
                let data = self::Data::from_single_for_list(entry.trim_start().to_string());
                out.push(data);
            }
            return Data::List(out);
        } else {
            return Data::String(value);
        }

    }
}

impl Data {
    pub fn is_null(&self) -> bool {
        match self {
            Self::Null() => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Self::String(_contents) => true,
            _ => false,
        }
    }
    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::String(contents) => Some(contents.to_owned()),
            _ => None,
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {
            Self::Bool(_contents) => true,
            _ => false,
        }
    }
    pub fn get_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(contents) => Some(contents.to_owned()),
            _ => None,
        }
    }
    pub fn is_float(&self) -> bool {
        match self {
            Self::Float(_contents) => true,
            _ => false,
        }
    }
    pub fn get_float(&self) -> Option<f64> {
        match self {
            Self::Float(contents) => Some(contents.to_owned()),
            _ => None,
        }
    }
    pub fn is_list(&self) -> bool {
        match self {
            Self::List(_contents) => true,
            _ => false,
        }
    }
    pub fn get_list(&self) -> Option<Vec<Data>> {
        match self {
            Self::List(contents) => Some(contents.to_owned()),
            _ => None,
        }
    }
    pub fn get_type(&self) -> String {
        match self {
            Self::List(_anything) => {
                return "List".to_string();
            },
            Self::Float(_anything) => {
                return "Float".to_string();
            },
            Self::Bool(_maybe) => {
                return "Bool".to_string();
            },
            Self::String(_anything) => {
                return "String".to_string();
            },
            Self::Null() => {
                return "Null".to_string();
            },
        }
    }
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
        if value.is_null() {
            return Ok(Self::Null());
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


