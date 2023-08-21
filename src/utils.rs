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

#[derive(Clone)]
pub struct UsizeRangeTupleContainer(pub (usize, usize));

impl ToString for UsizeRangeTupleContainer {
    fn to_string(&self) -> String {
        format!("({}, {})", self.0 .0, self.0 .1)
    }
}

// FIXME.
pub fn get_file_bound_to(bound_to: &Path, beginning_name: &str) -> File {
    File::create(format!(
        "{beginning_name}{:?}.txt",
        bound_to.file_stem().unwrap().to_str(),
    ))
    .unwrap()
}

pub fn range_parser(lower_and_upper: &str) -> Result<UsizeRangeTupleContainer, String> {
    let lower_and_upper: Vec<usize> = lower_and_upper
        .split(',')
        .map(|num| {
            num.parse()
                .expect("`--...-range` must be of the form: <lower bound>,<upper bound>.")
        })
        .collect();

    let (lower, upper) = (
        lower_and_upper.get(0).unwrap().to_owned(),
        lower_and_upper.get(1).unwrap().to_owned(),
    );

    if lower < upper {
        Ok(UsizeRangeTupleContainer((lower, upper)))
    } else {
        Err(format!(
            "<lower bound> (You provided: {lower}) must be lower than <upper bound> ({upper}).
You provided:
<lower bound>: {}
<upper bound>: {}",
            lower, upper
        ))
    }
}
