use json::JsonValue;

use crate::data::Data;

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

impl Default for ColumnData {
    fn default() -> Self {
        let aaa: Vec<Data> = Vec::new();
        return ColumnData{all_row_data: aaa};
    }
}

/* impl Default for Column {
    fn default() -> Self {
        
    }
} */

impl Column {
    pub fn from_neithdb_column_data(column_value: (&str, &JsonValue)) -> Self {
        let name = column_value.0.to_string();
        println!("NAME: {:?}", name);
        let data_object = column_value.1.clone();
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
