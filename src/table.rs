use json::JsonValue;

use crate::column::Column;


#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    name: String,
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

impl Table {
    pub fn new(name: String) -> Self {
        let columns: Vec<Column> = Vec::new();
        return Table {name, columns, };
    }
}


