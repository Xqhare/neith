use std::io::Error;

use json::JsonValue;

use crate::{column::Column, success::Success, data::Data, utils::util};


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
    pub fn new(name: String) -> Self {
        let columns: Vec<Column> = Vec::new();
        return Table {name, columns, };
    }
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
    pub fn new_data(&mut self, value: Vec<(String, Data)>) -> Result<Success, Error> {
        let mut success = true;
        for entry in value {
            let columnname = entry.0;
            let data = entry.1;
            let column_index = self.search_for_column(columnname)?;
            let column = self.columns[column_index].new_data(data);
            if column == Success::SuccessMessage(true) && success == true {
                success = true;
            } else {
                success = false;
            }
        }
        if success {
            return Ok(Success::SuccessMessage(true));
        } else {
            return Err(Error::other("Writing data went wrong!"));
        }
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
        let column_index = self.search_for_column(columnname)?;
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
}


