use std::{fs::File, path::Path};

// FIXME.
pub fn get_file_bound_to(bound_to: &Path, beginning_name: &str) -> File {
    File::create(format!(
        "{beginning_name}{:?}.txt",
        bound_to.file_stem().unwrap().to_str(),
    ))
    .unwrap()
}
