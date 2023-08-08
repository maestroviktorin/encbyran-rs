use rand::Rng;
use std::io::prelude::*;
use std::{
    fs::{read_to_string, File},
    path::Path,
};

pub fn cryptograph(
    path_to_file_to_encrypt: &Path,
    to_lower: bool,
    rm_punctuation: bool,
    shift: (usize, usize),
    encrypted_new_line_length: (usize, usize),
) {
    // IMPORTANT: Investigate topic of functions allocation, traits and functions borrowing.
    // INITIALLY PROVIDED BY: https://github.com/alexschrod (ilyvion on Discord).
    let perform_to_lower: fn(&str) -> String = if to_lower {
        |word: &str| word.to_lowercase()
    } else {
        |word: &str| word.to_string()
    };

    let perform_rm_punctuation: fn(String) -> String = if rm_punctuation {
        cleared
    } else {
        |word: String| word.to_string()
    };

    let origin_lines: Vec<Vec<String>> = read_to_string(&path_to_file_to_encrypt)
        .unwrap()
        .lines()
        .map(|line: &str| {
            line.trim()
                .split(" ")
                .map(perform_to_lower)
                .map(perform_rm_punctuation)
                .collect()
        })
        .collect();

    let mut encrypted: File = get_file_bound_to(&path_to_file_to_encrypt, "encrypted-");
    let mut decryptor: File = get_file_bound_to(&path_to_file_to_encrypt, "decryptor-for-");

    for line in origin_lines.iter() {
        encrypt_line(
            line,
            &mut encrypted,
            &mut decryptor,
            &shift,
            &encrypted_new_line_length,
        );
    }
}

fn cleared(word: String) -> String {
    let mut result: String = String::from(&word);
    let mut n: usize = word.len() - 1;

    while word.chars().nth(n).unwrap().is_ascii_punctuation() {
        result.remove(n);
        n -= 1;
    }

    result
}

fn get_file_bound_to(bound_to: &Path, beginning_name: &str) -> File {
    File::create(format!(
        "{beginning_name}{:?}{:?}",
        bound_to.file_stem().unwrap().to_str(),
        bound_to.extension().unwrap().to_str()
    ))
    .unwrap()
}

fn encrypt_line(
    line: &Vec<String>,
    encrypted: &mut File,
    decryptor: &mut File,
    shift: &(usize, usize),
    encrypted_new_line_length: &(usize, usize),
) {
    for word in line {
        encrypt_word(word, encrypted, decryptor, shift);
    }

    // Encrypt the new line notation using `encrypted_new_line_length`.
}

fn encrypt_word(word: &str, encrypted: &mut File, decryptor: &mut File, shift: &(usize, usize)) {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for byte in word.as_bytes() {
        let (key, action) = (rng.gen_range(shift.0..shift.1) as isize, rng.gen_bool(0.5));
        write!(decryptor, "{}", key.to_string()).unwrap();

        // TODO: Add 'action-words' written into `encrypted`.
        if action {
            write!(encrypted, "{}", (*byte as isize + key).to_string()).unwrap();
        } else {
            write!(encrypted, "{}", (*byte as isize - key).to_string()).unwrap();
        }
    }

    write!(encrypted, "\n").unwrap();
    write!(decryptor, "\n").unwrap();
}

fn get_encrypted_new_line(
    encrypted_new_line_length: &(usize, usize),
    shift: &(usize, usize),
) -> String {
    // Build an encrypted newline by joining some 'units'.
    String::from("")
}

fn get_encrypted_new_line_unit(shift: &(usize, usize)) -> String {
    // Produce a unit of an encrypted new line.
    String::from("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cleared() {
        assert_eq!(cleared("Hello!".to_string()), "Hello".to_string());
        assert_eq!(cleared("Hello...".to_string()), "Hello".to_string());
    }
}
