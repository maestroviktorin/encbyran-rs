use rand::seq::SliceRandom;
use std::collections::HashSet;

pub struct ActionVecs {
    plus: Vec<String>,
    minus: Vec<String>,
    new_line: Vec<String>,
}

impl ActionVecs {
    pub fn new(set_size: usize, word_max_size: usize) -> Self {
        let mut plus: ActionSet = get_random_set(set_size, word_max_size);
        let mut minus: ActionSet = get_random_set(set_size, word_max_size);
        let mut new_line: ActionSet = get_random_set(set_size, word_max_size);

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

    // TODO: Implement random sampling from the requested actions vector.
}

type ActionSet = HashSet<String>;

const ASCII_LETTERS: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

fn get_random_set(set_size: usize, word_max_size: usize) -> ActionSet {
    let mut result: ActionSet = HashSet::new();

    for _ in 0..set_size {
        result.insert(get_random_word(word_max_size));
    }

    result
}

fn get_random_word(word_max_size: usize) -> String {
    // https://docs.rs/rand/latest/rand/seq/index.html
    let mut result: String = String::new();
    let mut rng = rand::thread_rng();

    for _ in 1..word_max_size {
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
