use crate::entry_data::{EntryData, EntryType};
use crate::error::Error;
use std::fs::{File, FileType};
use std::io::Read;
use std::path::PathBuf;

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
