use std::io::Error;

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

    /// Used for converting Json to Neith data. Takes in the name of the column as an &str, along
    /// with the JsonValue
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

    /// Creates new data from an execute function.
    pub fn new_data(&mut self, value: Data) -> Result<Success, Error> {
        if self.unique {
            if self.contents.all_row_data.contains(&value) {
                return Err(Error::other(format!("This column ({:?}) is marked as unique and {:?} was found to be an entry already.", self.name, value)));
            }
        }
        return Ok(self.contents.new_data(value));
    }

    /// Deletes data from an execute function.
    pub fn delete_data(&mut self, index: usize) -> Success {
        return self.contents.delete_data(index);
    }

    /// Updates data from an execute function.
    pub fn update_data(&mut self, index: usize, value: Data) -> Result<Success, Error> {
        if self.unique {
            if self.contents.all_row_data.contains(&value) {
                return Err(Error::other(format!("This column ({:?}) is marked as unique and {:?} was found to be an entry already.", self.name, value)));
            }
        }
        return Ok(self.contents.update_data(index, value));
    }


    /// gets the minimum entry of a column
    pub fn min(&self) -> Success {
        return self.contents.min();
    }

    /// gets the maximum entry of a column
    pub fn max(&self) -> Success {
        return self.contents.max();
    }

}

impl ColumnData {

    /// Creates new data from an execute function.
    pub fn new_data(&mut self, value: Data) -> Success {
        self.all_row_data.push(value);
        return Success::SuccessMessage(true);
    }

    /// Deletes data from an execute function.
    pub fn delete_data(&mut self, index: usize) -> Success {
        let _ = self.all_row_data.remove(index);
        return Success::SuccessMessage(true);
    }

    /// Updates data from an execute function.
    pub fn update_data(&mut self, index: usize, value: Data) -> Success {
        let _ = self.delete_data(index);
        self.all_row_data.insert(index, value);
        return Success::SuccessMessage(true);
    }

    /// gets the minimum entry of a column
    pub fn min(&self) -> Success {
        let mut out = self.all_row_data.first().unwrap();
        for data in self.all_row_data.iter().skip(1) {
            if data < out {
                out = data;
            }
        }
        return Success::Result(vec![out.to_owned()]);
    }

    /// gets the maximum entry of a column
    pub fn max(&self) -> Success {
        let mut out = self.all_row_data.first().unwrap();
        for data in self.all_row_data.iter().skip(1) {
            if data > out {
                out = data;
            }
        }
        return Success::Result(vec![out.to_owned()]);
    }

}

