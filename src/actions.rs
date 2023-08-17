use rand::seq::SliceRandom;
use std::{collections::HashSet, fs, io::Write, path::Path};

pub struct ActionVecs {
    pub plus: Vec<String>,
    pub minus: Vec<String>,
    pub new_line: Vec<String>,
}

impl ActionVecs {
    pub fn new(approximate_action_set_size: usize, action_word_max_size: usize) -> Self {
        let mut plus: ActionSet = get_random_set(approximate_action_set_size, action_word_max_size);
        let mut minus: ActionSet =
            get_random_set(approximate_action_set_size, action_word_max_size);
        let mut new_line: ActionSet =
            get_random_set(approximate_action_set_size, action_word_max_size);

        // Solution obtained from
        // https://stackoverflow.com/questions/76860337/compound-hashset-operations-in-rust-or-how-to-get-an-explicit-difference-union
        plus.retain(|word: &String| minus.remove(word) | new_line.remove(word) | true);
        minus.retain(|word: &String| new_line.remove(word) | true);

        Self {
            plus: Vec::from_iter(plus.into_iter()),
            minus: Vec::from_iter(minus.into_iter()),
            new_line: Vec::from_iter(new_line.into_iter()),
        }
    }

    pub fn get_random_plus(&self) -> &String {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        self.plus.choose(&mut rng).unwrap()
    }

    pub fn get_random_minus(&self) -> &String {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        self.minus.choose(&mut rng).unwrap()
    }

    pub fn get_random_new_line(&self) -> &String {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

        self.new_line.choose(&mut rng).unwrap()
    }

    pub fn write_to(&self, to: &mut fs::File) {
        write!(to, "{:?}\n", self.plus).unwrap();
        write!(to, "{:?}\n", self.minus).unwrap();
    }

    pub fn read_from(from: &Path) -> Self {
        let source = fs::read_to_string(from).unwrap();

        Self {
            plus: obtain_action_vec(&source, 0),
            minus: obtain_action_vec(&source, 1),
            new_line: vec![],
        }
    }
}

type ActionSet = HashSet<String>;

const ASCII_LETTERS: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

fn get_random_set(approximate_action_set_size: usize, action_word_max_size: usize) -> ActionSet {
    let mut result: ActionSet = HashSet::new();

    for _ in 0..approximate_action_set_size {
        result.insert(get_random_word(action_word_max_size));
    }

    result
}

fn get_random_word(action_word_max_size: usize) -> String {
    // https://docs.rs/rand/latest/rand/seq/index.html
    let mut result: String = String::new();
    let mut rng = rand::thread_rng();

    for _ in 1..action_word_max_size {
        result.push(
            *ASCII_LETTERS
                .to_string()
                .into_bytes()
                .choose(&mut rng)
                .unwrap() as char,
        );
    }

    result
}

fn obtain_action_vec(source: &String, line_number: usize) -> Vec<String> {
    source
        .lines()
        .nth(line_number)
        .unwrap()
        .trim_matches(|chr: char| !chr.is_alphanumeric())
        .split(", ")
        .map(|action_word| String::from(action_word.trim_matches('"')))
        .collect()
}
