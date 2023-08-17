/*
Work in Progress.

META TODO:
   * Explore https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html to provide `encbyran` with CLI.
   * Rewrite https://github.com/maestroviktorin/encbyran-py in Rust.

TODO:
   * Extract `EncryptedByte` from `decryptograph.rs` to `utils.rs` and implement its usage in `cryptograph.rs` as well.

   * Fix names of files created during execution of the program,
     so that `decryptor-for-Some("sample").txt` became `decryptor-for-sample.txt`.

   * Write documentation.
*/

mod actions;
mod cryptograph;
mod decryptograph;
mod utils;

use std::path::Path;

fn main() {
    cryptograph::cryptograph(
        Path::new("./sample.txt"),
        false,
        true,
        (1, 200),
        (1, 15),
        10,
        10,
    );

    decryptograph::decryptograph(
        Path::new(r#"encrypted-Some("sample").txt"#),
        Path::new(r#"decryptor-for-Some("sample").txt"#),
    );
}
