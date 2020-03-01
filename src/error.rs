pub mod prelude {
    pub use super::{Error, Result, ServeError, ServeResult};
}

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

    #[fail(display = "string contains invalid unicode: {}", string)]
    InvalidUnicode { string: String },

    #[fail(display = "{} is not yet implemented, sorry!", feature_desc)]
    Unimplemented { feature_desc: String },
}
