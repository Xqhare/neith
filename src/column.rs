use json::JsonValue;

use crate::{data::Data, success::Success};

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    pub name: String,
    pub unique: bool,
    pub contents: ColumnData,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnData {
    pub all_row_data: Vec<Data>,
}

impl Default for Column {
    fn default() -> Self {
        let name = String::new();
        let unique = false;
        let contents = ColumnData::default();
        return Column{ name, unique, contents, };
    }
}

impl Default for ColumnData {
    fn default() -> Self {
        let row_data: Vec<Data> = Vec::new();
        return ColumnData{all_row_data: row_data};
    }
}

impl From<(String, bool)> for Column {
    fn from(value: (String, bool)) -> Self {
        let name = value.0;
        let unique = value.1;
        let contents = ColumnData::default();
        return Column {
            name,
            unique,
            contents,
        };
    }
}

impl Column {
    pub fn from_neithdb_column_data(column_value: (&str, &JsonValue)) -> Self {
        let name = column_value.0.to_string();
        let data_object = column_value.1.clone();
        let unique = data_object["unique"].as_bool().expect("Boolean not a boolean!");
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
        return Column{name, unique, contents: ColumnData { all_row_data}};
    }
    pub fn new_data(&mut self, value: Data) -> Success {
        return self.contents.new_data(value);
    }
    pub fn delete_data(&mut self, index: usize) -> Success {
        return self.contents.delete_data(index);
    }
}

impl ColumnData {
    pub fn new_data(&mut self, value: Data) -> Success {
        self.all_row_data.push(value);
        return Success::SuccessMessage(true);
    }
    pub fn delete_data(&mut self, index: usize) -> Success {
        let _ = self.all_row_data.remove(index);
        return Success::SuccessMessage(true);
    }
}
