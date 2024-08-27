use gen_data::scaffold;
use clap::Parser;

///Saffolds the the database schema into rust structs and typescript types
/// uses the sq


fn main() {
    scaffold::read_files()
}

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}