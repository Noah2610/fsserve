use crate::error::{Error, ServeError, ServeResult};
use std::fs::{File, FileType};
use std::io::Read;
use std::path::PathBuf;

enum EntryType {
    File,
    Directory,
}

impl ToString for EntryType {
    fn to_string(&self) -> String {
        String::from(match self {
            EntryType::File => "f",
            EntryType::Directory => "d",
        })
    }
}

use std::convert::TryFrom;

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

/// Serves a directory or file (`path`), and holds
/// some options for serving (`ServerOptions`).
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Server {
    path:    PathBuf,
    #[builder(default)]
    options: ServerOptions,
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub fn serve(self) -> Result<(), Error> {
        let root_file_type = self.path.metadata()?.file_type();

        if root_file_type.is_file() {
            self.serve_file()
        } else if root_file_type.is_dir() {
            self.serve_directory()
        } else {
            unreachable!(
                "PathBuf::metadata should follow symlinks, so the FileType \
                 has to be either a file or a directory.
                 Execution should never get to this point!"
            )
        }
    }

    // TODO
    fn serve_file(self) -> Result<(), Error> {
        unimplemented!()
    }

    fn serve_directory(self) -> Result<(), Error> {
        Ok(())
    }
}

// TODO
#[derive(Default)]
pub struct ServerOptions;

/// This data structure represents either a file or a directory.
/// It is returned from the endpoint as JSON.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct EntryData {
    name:      String,
    path:      PathBuf,
    file_type: FileType,
}
