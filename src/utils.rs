use std::{
    fmt::{Display, Formatter},
    fs::File,
    path::Path,
};

pub struct EncryptedByte {
    pub action_word: Option<String>,
    pub value: Option<isize>,
}

impl EncryptedByte {
    pub fn empty() -> Self {
        Self {
            action_word: None,
            value: None,
        }
    }

    pub fn filled(action_word: String, value: isize) -> Self {
        Self {
            action_word: Some(action_word),
            value: Some(value),
        }
    }
}

impl Display for EncryptedByte {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let msg: &str = "Cannot write `EncryptedByte` with empty fields.";
        write!(
            f,
            "{}{} ",
            self.action_word.as_ref().expect(msg),
            self.value.expect(msg)
        )
    }
}

pub fn get_file_bound_to(bound_to: &Path, beginning_name: &str) -> File {
    File::create(format!(
        "{beginning_name}{:?}.txt",
        bound_to.file_stem().unwrap().to_str(),
    ))
    .unwrap()
}
