use crate::entry_data::{EntryData, EntryType};
use crate::error::prelude::*;
use std::convert::{TryFrom, TryInto};
use std::path::PathBuf;

/// Serves a directory or file (`path`), and holds
/// some options for serving (`ServerOptions`).
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Server {
    root:    PathBuf,
    #[builder(default)]
    options: ServerOptions,
}

impl Server {
    pub fn builder() -> ServerBuilder {
        ServerBuilder::default()
    }

    pub fn serve(self) -> Result {
        let root_type = EntryType::try_from(&self.root)?;

        match root_type {
            EntryType::File => self.serve_file(),
            EntryType::Directory => self.serve_directory(),
        }
    }

    // TODO
    fn serve_file(self) -> Result {
        Err(ServeError::Unimplemented {
            feature_desc: "serving a file".to_string(),
        }
        .into())
    }

    fn serve_directory(self) -> Result {
        let entry_data: EntryData = (&self.root).try_into()?;
        println!("{}", serde_json::ser::to_string_pretty(&entry_data)?);
        Ok(())
    }
}

#[derive(Default, Builder)]
#[builder(pattern = "owned")]
pub struct ServerOptions {
    interfaces: Vec<&'static str>,
}

impl ServerOptions {
    pub fn builder() -> ServerOptionsBuilder {
        ServerOptionsBuilder::default()
    }
}
