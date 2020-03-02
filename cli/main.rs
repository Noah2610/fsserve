extern crate fsserve;

use fsserve::server::{Server, ServerOptions};
use std::env;
use std::path::PathBuf;

fn main() -> Result<(), String> {
    let mut args = env::args();
    let _prog_name = args.next();
    let path = PathBuf::from(
        args.next()
            .expect("First argument needs to be filepath to host\nex.: \".\""),
    );
    let host = args.next().expect(
        "Second argument needs to be the address to bind the server to\nex.: \
         \"localhost\"",
    );
    let port = args.next().expect(
        "Second argument needs to be the port to bind the server to\nex.: \
         \"8080\"",
    );

    Server::builder()
        .root(path)
        .options(
            ServerOptions::builder()
                .host(host)
                .port(port)
                .build()
                .expect("Couldn't build ServerOptions"),
        )
        .build()?
        .serve()
        .expect("Error serving Server");
    Ok(())
}
