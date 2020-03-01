extern crate fsserve;

use fsserve::server::Server;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let path = PathBuf::from(".");
    Server::builder().path(path).build()?.serve();
    Ok(())
}
