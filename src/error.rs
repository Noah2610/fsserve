use std::fmt;

pub use failure::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;
pub type ServeResult<T = ()> = std::result::Result<T, ServeError>;

#[derive(Debug, Fail)]
pub enum ServeError {
    #[fail(
        display = "invalid file type for serving, can only be file or \
                   directory: {}",
        file_type
    )]
    InvalidFileType { file_type: String },
}
