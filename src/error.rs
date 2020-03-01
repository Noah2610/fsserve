use std::fmt;

pub use failure::Error;

pub type ServeResult<T = ()> = Result<T, Error>;

#[derive(Debug, Fail)]
pub enum ServeError {}

impl fmt::Display for ServeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}
