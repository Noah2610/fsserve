use crate::error::{ServeError, ServeResult};
use std::convert::TryFrom;
use std::fmt;
use std::fs::FileType;

/// An `EntryType` represents either a `File` or a `Directory`.
/// It can be converted into a `String` (`ToString`),
/// which returns either an `"f"` or an `"d"`, depending
/// on the type.
pub enum EntryType {
    File,
    Directory,
}

impl TryFrom<FileType> for EntryType {
    type Error = ServeError;

    fn try_from(file_type: FileType) -> ServeResult<Self> {
        if file_type.is_file() {
            Ok(EntryType::File)
        } else if file_type.is_dir() {
            Ok(EntryType::Directory)
        } else {
            Err(ServeError::InvalidFileType {
                file_type: format!("{:?}", file_type),
            })
        }
    }
}

impl fmt::Display for EntryType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            EntryType::File => "f",
            EntryType::Directory => "d",
        })
    }
}
