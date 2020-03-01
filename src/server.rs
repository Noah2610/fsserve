use crate::entry_data::{EntryData, EntryType};
use crate::error::prelude::*;
use simple_server::{ResponseBuilder, ResponseResult, Server as HttpServer};
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
        let serve_options = self.options;

        let server =
            HttpServer::new(move |request, response| -> ResponseResult {
                eprintln!("Request: {} {}", request.method(), request.uri());

                if request.method() == "GET" {
                    let path = self.root.join(&request.uri().path()[1 ..]);
                    dbg!(&path);

                    let entry_data_json_res: Result<String> =
                        (&path).try_into().and_then(|entry_data: EntryData| {
                            Ok(serde_json::ser::to_string_pretty(&entry_data)?)
                        });

                    match entry_data_json_res {
                        Ok(entry_data_json) => {
                            response_ok(response, entry_data_json)
                        }

                        // TODO: Return more appropriate status codes,
                        //       depending on the type of error.
                        Err(e) => {
                            eprintln!("500: {}", e);
                            response_internal_server_error(response)
                        }
                    }
                } else {
                    response_method_not_allowed(response)
                }
            });

        server.listen(serve_options.host, serve_options.port);
    }
}

// TODO move refactor these helper functions...
fn response_ok(mut response: ResponseBuilder, body: String) -> ResponseResult {
    Ok(response.status(200).body(body.as_bytes().to_vec())?)
}

fn response_bad_request(mut response: ResponseBuilder) -> ResponseResult {
    Ok(response
        .status(400)
        .body("400 Bad Request".as_bytes().to_vec())?)
}

fn response_forbidden(mut response: ResponseBuilder) -> ResponseResult {
    Ok(response
        .status(403)
        .body("403 Forbidden".as_bytes().to_vec())?)
}

fn response_method_not_allowed(
    mut response: ResponseBuilder,
) -> ResponseResult {
    Ok(response
        .status(405)
        .body("405 Method Not Allowed".as_bytes().to_vec())?)
}

fn response_internal_server_error(
    mut response: ResponseBuilder,
) -> ResponseResult {
    Ok(response
        .status(500)
        .body("500 Internal Server Error".as_bytes().to_vec())?)
}

#[derive(Clone, Copy, Builder)]
#[builder(pattern = "owned")]
pub struct ServerOptions {
    host: &'static str,
    port: &'static str,
}

impl Default for ServerOptions {
    fn default() -> Self {
        Self {
            host: "localhost",
            port: "8080",
        }
    }
}

impl ServerOptions {
    pub fn builder() -> ServerOptionsBuilder {
        ServerOptionsBuilder::default()
    }
}
