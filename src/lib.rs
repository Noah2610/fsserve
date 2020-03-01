#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate failure;
extern crate http;
extern crate serde_json;
#[macro_use]
extern crate serde;

pub mod entry_data;
pub mod error;
pub mod server;
