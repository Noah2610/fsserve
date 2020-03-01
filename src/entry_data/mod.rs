pub mod entry_type;

pub use entry_type::EntryType;

use std::path::PathBuf;

/// This data structure represents either a file or a directory.
/// It is returned from the endpoint as JSON.
#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct EntryData {
    name:      String,
    path:      PathBuf,
    file_type: EntryType,
}
