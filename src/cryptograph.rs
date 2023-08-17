use super::utils::get_file_bound_to;

use rand::Rng;
use std::io::prelude::*;
use std::{
    fs::{read_to_string, File},
    path::Path,
};

use super::actions::ActionVecs;

pub fn cryptograph(
    path_to_file_to_encrypt: &Path,
    to_lower: bool,
    rm_punctuation: bool,
    shift: (usize, usize),
    encrypted_new_line_length: (usize, usize),
    approximate_action_set_size: usize,
    action_word_max_size: usize,
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

    let action_vecs: ActionVecs =
        ActionVecs::new(approximate_action_set_size, action_word_max_size);
    action_vecs.write_to(&mut decryptor);

    for line in origin_lines.iter() {
        encrypt_line(
            line,
            &mut encrypted,
            &mut decryptor,
            &shift,
            &encrypted_new_line_length,
            &action_vecs,
        );
    }
}

fn cleared(word: String) -> String {
    /*
    Possible improvement:
        Add configurability of leaving non-alphanumerical characters located amidst the `word`,
        so that "sea-buckthorn!" turns into "sea-buckthorn", but not into "seabuckthorn".
    */
    word.chars()
        .filter(|chr: &char| chr.is_alphanumeric())
        .collect()
}

fn encrypt_line(
    line: &Vec<String>,
    encrypted: &mut File,
    decryptor: &mut File,
    shift: &(usize, usize),
    encrypted_new_line_length: &(usize, usize),
    action_vecs: &ActionVecs,
) {
    for word in line {
        encrypt_word(word, encrypted, decryptor, shift, action_vecs);
    }

    let encrypted_new_line: String =
        get_encrypted_new_line(encrypted_new_line_length, shift, action_vecs);
    write!(encrypted, "{}\n", encrypted_new_line).unwrap();
    write!(decryptor, "0\n").unwrap();
}

fn encrypt_word(
    word: &str,
    encrypted: &mut File,
    decryptor: &mut File,
    shift: &(usize, usize),
    action_vecs: &ActionVecs,
) {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    for byte in word.as_bytes() {
        let (key, action) = (rng.gen_range(shift.0..shift.1) as isize, rng.gen_bool(0.5));
        write!(decryptor, "{} ", key.to_string()).unwrap();

        // Possible improvement:
        //  Reduce this `if-else` block using `EncryptedByte` struct.
        if action {
            write!(
                encrypted,
                "{}{} ",
                action_vecs.get_random_plus(),
                (*byte as isize + key).to_string()
            )
            .unwrap();
        } else {
            write!(
                encrypted,
                "{}{} ",
                action_vecs.get_random_minus(),
                (*byte as isize - key).to_string()
            )
            .unwrap();
        }
    }

    write!(encrypted, "\n").unwrap();
    write!(decryptor, "\n").unwrap();
}

fn get_encrypted_new_line(
    encrypted_new_line_length: &(usize, usize),
    shift: &(usize, usize),
    action_vecs: &ActionVecs,
) -> String {
    // Build an encrypted newline by joining some 'units'.
    let mut result: Vec<String> = Vec::new();
    let finite_encrypted_new_line_length: usize =
        rand::thread_rng().gen_range(encrypted_new_line_length.0..encrypted_new_line_length.1);

    for _ in 0..finite_encrypted_new_line_length {
        result.push(get_encrypted_new_line_unit(shift, action_vecs));
    }

    result.join(" ")
}

fn get_encrypted_new_line_unit(shift: &(usize, usize), action_vecs: &ActionVecs) -> String {
    // Produce a unit of an encrypted new line.
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut shift_: isize = rng.gen_range(shift.0..shift.1) as isize;
    shift_ = if rng.gen_bool(0.5) { shift_ } else { -shift_ };

    let new_line_notation: String = action_vecs.get_random_new_line().clone();
    let phantom_number = new_line_notation
        .chars()
        .nth(rng.gen_range(0..new_line_notation.len()))
        .unwrap()
        .to_string()
        .as_bytes()
        .get(0)
        .unwrap()
        .clone() as isize
        + shift_;

    format!("{}{}", new_line_notation, phantom_number)
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
