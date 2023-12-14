use crate::data::Data;

#[derive(Clone, PartialEq, Debug)]
/// Neith will either return a error, or this `Success` enum.
/// `Success` can contain the general `SucessMessage` wrapping a boolean or the requested Result,
/// wrapping a vector of `Data`.
pub enum Success {
    /// The wrapped boolean can be disregarded, it is never set to `false` but helps with debugging.
    SuccessMessage(bool),
    /// The wrapped vector contains the requested data, these are often nested vectors!
    Result(Vec<Data>),
}

impl Success {
    /// Function to inspect if Success is wrapping a result.
    ///
    /// ## Returns
    /// Returns `true` if Success is wrapping a result, `false` otherwise.
    pub fn is_result(&self) -> bool {
        match self {
            Success::Result(_data) => true,
            _ => false,
        }
    }
    /// Function to get the contents of a Success wrapping a result.
    ///
    /// ## Returns
    /// `Some(Vec<Data>)` if a result exists, `None` otherwise.
    pub fn get_result(&self) -> Option<Vec<Data>> {
        match self {
            Success::Result(answ) => return Some(answ.to_owned()),
            _ => None,
        }
    }
}
