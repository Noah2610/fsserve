use std::fmt;

pub use failure::Error;

pub type ServeResult<T = ()> = Result<T, ServeError>;

#[derive(Debug, Fail)]
pub enum ServeError {
    #[fail(
        display = "invalid file type for serving, can only be file or \
                   directory: {}",
        file_type
    )]
    InvalidFileType { file_type: String },
}
