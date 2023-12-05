use crate::data::Data;

#[derive(Clone, PartialEq, Debug)]
pub enum Success {
    SuccessMessage(bool),
    Result(Vec<Data>),
    Length(usize),
}

