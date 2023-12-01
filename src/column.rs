use json::JsonValue;

use crate::data::Data;

#[derive(Clone, Debug, PartialEq)]
pub struct Column {
    name: String,
    unique: bool,
    contents: ColumnData,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColumnData {
    all_row_data: Vec<Data>,
}

impl Default for ColumnData {
    fn default() -> Self {
        let row_data: Vec<Data> = Vec::new();
        return ColumnData{all_row_data: row_data};
    }
}

impl Default for Column {
    fn default() -> Self {
        let name = String::new();
        let unique = false;
        let contents = ColumnData::default();
        return Column{ name, unique, contents, };
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
}
