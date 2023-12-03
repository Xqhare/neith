use std::io::Error;

use json::JsonValue;

use crate::{column::Column, success::Success, data::Data, utils::util};


#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub name: String,
    columns: Vec<Column>,
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
    pub fn new_data(&mut self, value: Vec<(String, Data)>) -> Result<Success, Error> {
        let mut success = true;
        for entry in value {
            let columnname = entry.0;
            let data = entry.1;
            let column_index = self.search_for_column(columnname)?;
            let mut column = self.columns[column_index].clone();
            let new = column.new_data(data);
            if new == Success::SuccessMessage(true) && success == true {
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
}


