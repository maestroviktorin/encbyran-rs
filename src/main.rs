// TODO:
//    * Explore https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html to provide `encbyran` with CLI.
//    * Rewrite https://github.com/maestroviktorin/encbyran-py in Rust.

// Work in Progress.

pub mod actions;
pub mod cryptograph;

use std::path::Path;

fn main() {
    cryptograph::cryptograph(
        Path::new("./sample.txt"),
        false,
        false,
        (1, 200),
        (1, 15),
        10,
        10,
    );
}
