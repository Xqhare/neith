#[derive(Clone)]
pub struct Column {
    name: String,
    unit_type: String,
    contents: ColumnData,
}

#[derive(Clone)]
pub struct ColumnData {
    all_row_data: Vec<Data>,
}

#[derive(Clone)]
pub enum Data {
    List,
    Int,
    Float,
    Bool,
    String,
}
