#[derive(Clone, PartialEq, Eq)]
pub enum Success {
    SuccessMessage(bool),
    Result(),
}

