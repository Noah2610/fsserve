use crate::error::ServeResult;
use std::fs::File;
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

    pub fn serve(self) -> ServeResult {
        let root_file_type = self.path.metadata()?;
        Ok(())
    }

    fn serve_file(self) {
    }

    fn serve_directory(self) {
    }
}

// TODO
#[derive(Default)]
pub struct ServerOptions;
