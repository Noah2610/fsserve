pub mod entry_type;

pub use entry_type::EntryType;

use crate::error::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::ffi::OsStr;
use std::path::PathBuf;

const ROOT_PATH: &str = "/";

/// This data structure represents either a file or a directory.
/// It is returned from the endpoint as JSON.
#[derive(Builder, Serialize)]
#[builder(pattern = "owned")]
pub struct EntryData {
    /// The filename. With trailing "/" if it is a directory.
    /// If this is the root entry, then the name is "/".
    name:       String,
    /// The full path to this entry,
    /// with which it can be accessed through the server.
    path:       String,
    /// If this entry is a file or a directory.
    #[serde(rename = "type")]
    entry_type: EntryType,
    /// Vec of entry paths to this entry's child items.
    /// Is only Some if this is a directory, is None if it is a file.
    entries:    Option<Vec<String>>,
}

impl TryFrom<&PathBuf> for EntryData {
    type Error = Error;

    fn try_from(path_buf: &PathBuf) -> Result<Self> {
        let root_os_str = &OsStr::new(ROOT_PATH);

        let name =
            os_str_to_string(path_buf.file_name().unwrap_or(root_os_str))?;
        let path = os_str_to_string(path_buf.as_os_str())?;
        let entry_type = path_buf.try_into()?;
        let entries = if let EntryType::Directory = &entry_type {
            let items = Result::from(
                path_buf
                    .read_dir()?
                    .map(|item| os_str_to_string(item?.path().as_os_str()))
                    .collect::<std::result::Result<_, _>>(),
            )?;
            Some(items)
        } else {
            None
        };

        Ok(Self {
            name,
            path,
            entry_type,
            entries,
        })
    }
}

fn os_str_to_string(os_str: &OsStr) -> Result<String> {
    Ok(os_str
        .to_str()
        .ok_or(Error::from(ServeError::InvalidUnicode {
            string: format!("{:?}", os_str),
        }))?
        .to_string())
}
