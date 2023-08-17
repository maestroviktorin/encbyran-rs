use super::{actions::ActionVecs, utils::get_file_bound_to};

use std::{fs, io::Write, path::Path};

use regex::Regex;

struct EncryptedByte {
    action_word: String,
    value: isize,
}

pub fn decryptograph(path_to_file_to_decrypt: &Path, path_to_decryptor: &Path) {
    let decryptor = fs::read_to_string(path_to_decryptor).unwrap();
    let mut decrypted: fs::File = get_file_bound_to(&path_to_file_to_decrypt, "decrypted-");

    let re_action_word: Regex = Regex::new(r"[a-zA-Z]+").unwrap();
    let re_number: Regex = Regex::new(r"-?\d+").unwrap();
    let encrypted_words: Vec<Vec<EncryptedByte>> = fs::read_to_string(path_to_file_to_decrypt)
        .unwrap()
        .lines()
        .map(|line: &str| {
            line.trim()
                .split(" ")
                .map(|char| EncryptedByte {
                    action_word: re_action_word.find(char).unwrap().as_str().to_string(),
                    value: re_number.find(char).unwrap().as_str().parse().unwrap(),
                })
                .collect()
        })
        .collect();

    let action_vecs: ActionVecs = ActionVecs::read_from(path_to_decryptor);

    let keys: Vec<Vec<isize>> = decryptor
        .lines()
        .map(String::from)
        .skip_while(|line| line.starts_with("["))
        .map(|line| {
            line.trim()
                .split(" ")
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    for word_i in 0..keys.len() {
        if keys[word_i][0] == 0 {
            write!(decrypted, "\n").unwrap();
            continue;
        }

        let mut word_as_bytes: Vec<u8> = vec![];
        for byte_i in 0..keys[word_i].len() {
            let current_byte: &EncryptedByte = &encrypted_words[word_i][byte_i];

            if action_vecs.plus.contains(&current_byte.action_word) {
                word_as_bytes.push((current_byte.value - keys[word_i][byte_i] as isize) as u8);
            }

            if action_vecs.minus.contains(&current_byte.action_word) {
                word_as_bytes.push((current_byte.value + keys[word_i][byte_i] as isize) as u8);
            }
        }

        write!(decrypted, "{} ", String::from_utf8_lossy(&word_as_bytes)).unwrap();
    }
}
