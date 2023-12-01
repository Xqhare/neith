use json::JsonValue;

use crate::column::Column;


#[derive(Clone, Debug)]
pub struct Table {
    columns: Column,
}

fn decode_json_table(table_contents: JsonValue) {
    for entry in table_contents.entries() {
        // println!("{:?}", entry);
        let test = Column::from_neith_json_column(entry);
        println!("------------------");
        println!("{:?}", test);
        println!("------------------");
    }
}
