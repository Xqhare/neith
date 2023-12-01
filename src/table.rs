use json::JsonValue;

use crate::column::Column;


#[derive(Clone, Debug)]
pub struct Table {
    name: String,
    columns: Vec<Column>,
}

impl Default for Table {
    fn default() -> Self {
        unimplemented!()
    }
}

impl Table {
    pub fn from_neithdb_table_data(input_data: (&str, &JsonValue)) -> Self {
        let name = input_data.0.to_string();
        let mut out: Vec<Column> = Vec::new();
        for column in input_data.1.entries() {
            out.push(Column::from_neithdb_column_data(column));
        }
        return Table{name, columns: out};
    }
}


