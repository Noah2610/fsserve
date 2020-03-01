use crate::error::{Error, Result, ServeError};
use std::convert::TryFrom;
use std::fmt;
use std::fs::FileType;
use std::path::PathBuf;

/// An `EntryType` represents either a `File` or a `Directory`.
/// It can be converted into a `String` (`ToString`),
/// which returns either an `"f"` or an `"d"`, depending
/// on the type.
pub enum EntryType {
    File,
    Directory,
}

impl TryFrom<FileType> for EntryType {
    type Error = Error;
    fn try_from(file_type: FileType) -> Result<Self> {
        if file_type.is_file() {
            Ok(EntryType::File)
        } else if file_type.is_dir() {
            Ok(EntryType::Directory)
        } else {
            Err(ServeError::InvalidFileType {
                file_type: format!("{:?}", file_type),
            }
            .into())
        }
    }
}

impl TryFrom<&PathBuf> for EntryType {
    type Error = Error;
    fn try_from(path: &PathBuf) -> Result<Self> {
        Self::try_from(path.metadata()?.file_type())
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
