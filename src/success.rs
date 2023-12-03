use crate::data::Data;

#[derive(Clone, PartialEq)]
pub enum Success {
    SuccessMessage(bool),
    Result(Vec<Data>),
}

