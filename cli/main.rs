extern crate fsserve;

use fsserve::server::Server;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let path = PathBuf::from(".");
    Server::builder().root(path).build()?.serve().unwrap();
    Ok(())
}
