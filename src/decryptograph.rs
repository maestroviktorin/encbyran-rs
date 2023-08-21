use super::{
    actions::ActionVecs,
    utils::{get_file_bound_to, EncryptedByte},
};

use std::{fs, io::Write, path::Path};

use regex::Regex;

/// Decrypts the Encrypted File encrypted by [`cryptograph`](super::cryptograph::cryptograph) using Decryptor.
///
/// ### Parameters
///
/// `path_to_file_to_decrypt`: Location of the Encrypted File to be decrypted.
///
/// `path_to_decryptor`: Location of Decryptor to the Encrypted File.
///
/// ### Examples
///
/// ```rust
/// use std::path::Path;
///
/// decryptograph(
///     Path::new("./encrypted-password.txt"), // This file was produced by the `cryptograph`. Now it will be decrypted.
///     Path::new("./decryptor-for-password.txt") // This file was produced by the `cryptograph`. Now it will be used to decrypt the Encrypted File passed above.
/// )
/// ```
pub fn decryptograph(path_to_file_to_decrypt: &Path, path_to_decryptor: &Path) {
    // Regular expressions that are used to parse Encrypted File.
    let re_action_word = Regex::new(r"[a-zA-Z]+").unwrap();
    let re_number = Regex::new(r"-?\d+").unwrap();

    // Parsing Encrypted File.
    let encrypted_words: Vec<Vec<EncryptedByte>> = fs::read_to_string(path_to_file_to_decrypt)
        .unwrap()
        .lines()
        .map(|line: &str| {
            line.trim()
                .split(" ")
                .map(|byte| {
                    EncryptedByte::filled(
                        re_action_word.find(byte).unwrap().as_str().to_string(),
                        re_number.find(byte).unwrap().as_str().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    // Getting `ActionVec`s from the first two lines of Decryptor.
    let action_vecs = ActionVecs::read_from(path_to_decryptor);

    // Getting keys from Decryptor.
    let decryptor = fs::read_to_string(path_to_decryptor).unwrap();
    let keys: Vec<Vec<isize>> = decryptor
        .lines()
        .map(String::from)
        // First two lines of Decryptor are `ActionVec`s, they are skipped.
        .skip_while(|line| line.starts_with("["))
        .map(|line| {
            line.trim()
                .split(" ")
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    let mut decrypted = get_file_bound_to(&path_to_file_to_decrypt, "decrypted-");
    for word_i in 0..keys.len() {
        // While in Encrypted File each new line is just a meaningless set of pseudo encrypted bytes,
        // in Decryptor each new line is clear `0`.
        if keys[word_i][0] == 0 {
            write!(decrypted, "\n").unwrap();
            continue;
        }

        // Decrypting each real (non-pseudo) encrypted byte one by one.
        let mut word_as_bytes: Vec<u8> = vec![];
        for byte_i in 0..keys[word_i].len() {
            let current_byte: &EncryptedByte = &encrypted_words[word_i][byte_i];

            if action_vecs
                .plus
                .contains(&current_byte.action_word.as_ref().unwrap())
            {
                word_as_bytes
                    .push((current_byte.value.unwrap() - keys[word_i][byte_i] as isize) as u8);
            }

            if action_vecs
                .minus
                .contains(&current_byte.action_word.as_ref().unwrap())
            {
                word_as_bytes
                    .push((current_byte.value.unwrap() + keys[word_i][byte_i] as isize) as u8);
            }
        }

        write!(decrypted, "{} ", String::from_utf8_lossy(&word_as_bytes)).unwrap();
    }
}
