use std::io::Error;

use json::JsonValue;

use crate::{column::Column, success::Success, data::Data};

#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub columns: Vec<Column>,
}

impl Default for Table {
    fn default() -> Self {
        let name = String::new();
        let columns: Vec<Column> = Vec::new();
        return Table {name, columns, };
    }
}

impl From<(&str, &JsonValue)> for Table {
    fn from(value: (&str, &JsonValue)) -> Self {
        let name = value.0.to_string();
        let mut out: Vec<Column> = Vec::new();
        for column in value.1.entries() {
            out.push(Column::from_neithdb_column_data(column));
        }
        return Table{name, columns: out};
    }
}

impl From<(String, Vec<(String, bool)>)> for Table {
    fn from(value: (String, Vec<(String, bool)>)) -> Self {
        let tablename = value.0;
        let columns_in = value.1;
        let mut columns: Vec<Column> = Vec::new();
        for entry in columns_in {
            let name = entry.0;
            let unique = entry.1;
            let new_column = Column::from((name, unique));
            columns.push(new_column);
        }
        return Table {
            name: tablename,
            columns,
        };
    }
}

impl Table {
    pub fn new_columns(&mut self, value: Vec<(String, bool)>) -> Success {
        for entry in value {
            let new_column = Column::from(entry);
            self.columns.push(new_column);
        }
        return Success::SuccessMessage(true);
    }
    pub fn delete_data(&mut self, indicies: Vec<usize>) -> Result<Success, Error> {
        for index in indicies {
            for column in &mut self.columns {
                column.delete_data(index);
            }
        }
        return Ok(Success::SuccessMessage(true));
    }
    pub fn update_data(&mut self, value: Vec<(String, Data)>, indicies: Vec<usize>) -> Result<Success, Error> {
        let name_vec: Vec<String> = value.iter().map(|entry| {entry.0.clone()}).collect();
        for column in &mut self.columns {
            if name_vec.contains(&column.name) {
                for index in indicies.clone() {
                    for entry in value.clone() {
                        if column.name == entry.0 {
                            let _ = column.update_data(index, entry.1)?;
                        }
                    }
                }
            }
        }
        return Ok(Success::SuccessMessage(true));
    }
    pub fn select_data(self, coulumn_names: Vec<String>, indicies: Vec<usize>) -> Success {
        let mut found_data: Vec<Data> = Vec::new();
        if coulumn_names.contains(&"*".to_string()) {
            for column in self.columns {
                for index in indicies.clone() {
                    found_data.push(column.contents.all_row_data[index].clone());
                }
            }
        } else {
            for column in self.columns {
                if coulumn_names.contains(&column.name) {
                    for index in indicies.clone() {
                        found_data.push(column.contents.all_row_data[index].clone());
                    }
                }
            }
        }
        return Success::Result(found_data);
    }
    pub fn new_data(&mut self, value: Vec<(String, Data)>) -> Result<Success, Error> {
        let name_vec: Vec<String> = value.iter().map(|entry| {entry.0.clone()}).collect();
        for column in &mut self.columns {
            if name_vec.contains(&column.name) {
                for entry in value.clone() {
                    if column.name == entry.0 {
                        let _answ = column.new_data(entry.1)?;
                    }
                }
            } else {
                let _ = column.new_data(Data::Null())?;
            }
        }
        return Ok(Success::SuccessMessage(true));
    }
    pub fn search_for_column(&self, columnname: String) -> Result<usize, Error> {
        let mut counter: usize = 0;
        for entry in &self.columns {
            if entry.name.eq(&columnname) {
                return Ok(counter);
            }
            counter += 1;
        }
        return Err(Error::other(format!("Table with name {} not found.", columnname)));
    }
    pub fn delete_column(&mut self, columnname: String) -> Result<Success, Error> {
        let column_index = self.search_for_column(columnname)?;
        let _ = self.columns.remove(column_index);
        return Ok(Success::SuccessMessage(true));
    }
    /// Returns the index of the data in the column.
    pub fn search_column_data(&self, columnname: String, data: Data) -> Result<Vec<usize>, Error> {
        let column_index = self.search_for_column(columnname.clone())?;
        let mut out: Vec<usize> = Vec::new();
        let mut counter: usize = 0;
        for entry in self.columns[column_index].contents.all_row_data.clone() {
            if entry == data {
                out.push(counter);
            }
            counter += 1;
        }
        if out.len() > 0 {
            return Ok(out);
        } else {
            return Err(Error::other("No data found!"));
        }
    }
    pub fn len(&self) -> usize {
        return self.columns[0].contents.all_row_data.len();
    }
}

