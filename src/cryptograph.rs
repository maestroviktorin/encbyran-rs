use super::{
    actions::ActionVecs,
    utils::{get_file_bound_to, EncryptedByte},
};
use rand::Rng;
use std::{
    fs::{read_to_string, File},
    io::prelude::*,
    path::Path,
};

/// Encrypts text file at `path_to_file_to_encrypt` accounting all the provided parameters.
/// Encrypted File can be decrypted using [`decryptograph`](super::decryptograph::decryptograph).
///
/// ### Parameters
///
/// `path_to_file_to_encrypt`: Location of the file to be encrypted.
///
/// `to_lower`: Whether to bring all the letters to the lowercase or not.
///
/// `rm_punctuation`: Whether to omit all the punctuation characters or not.
///
/// `shift_range`: Maximum and minimum shift of each byte value.
/// E.g. with this equal to `(1, 10)` word `"AAA"` (`[65, 65, 65]` in bytes representation)
/// might look like `"7K>"` (`[55, 75, 62]`).
///
/// `encrypted_new_line_length_range`: Maximum and minimum length of a pseudo-word denoting a new line.
/// E.g. with this equal to (1, 3) `'\n'` might look like `"SHqJY98 KQqrRHqq-6"`
/// as it would be a real word consisting of 2 letters.
///
/// `approximate_action_set_size`: Approximate size of a set of special randomly generated "words"
/// indicating the direction of the byte value shift.
///
/// `action_word_max_size`: Maximum size of a special randomly generated "word"
/// indicating the direction of the byte value shift.
///  
/// ### Examples
///
/// ```rust, ignore
/// use std::path::Path;
///
/// cryptograph(
///     Path::new("./password.txt"), // This file will be encrypted.
///     false, // Remain all the letters as they are...
///     false, // and punctuation characters as well (we encrypt a password, it may contain bangs, dashes and so on...).
///     (1, 200) // All the byte values of characters will be mangled by so much.
///     (1, 15) // New line will be represented as a normal word.
///     10 // About by so much of different special "words" indicating the direction of the shift will be randomly generated...
///     10 // and no one of them will be longer than by so much.
/// )
/// ```
///
/// The bigger numbers you provide, the more secure encryption you receive.
pub fn cryptograph(
    path_to_file_to_encrypt: &Path,
    to_lower: bool,
    rm_punctuation: bool,

    // Possible improvement:
    //  Implement verification of `..._range` parameters
    //  so that (a, b) is invalid if a <= b.
    shift_range: (usize, usize),
    encrypted_new_line_length_range: (usize, usize),

    approximate_action_set_size: usize,
    action_word_max_size: usize,
) {
    // IMPORTANT: Investigate topic of functions allocation, traits and functions borrowing.
    // INITIALLY PROVIDED BY: https://github.com/alexschrod (ilyvion on Discord).
    let perform_to_lower: &dyn Fn(&str) -> String = if to_lower {
        &|word: &str| word.to_lowercase()
    } else {
        &|word: &str| word.to_string()
    };

    let perform_rm_punctuation: &dyn Fn(String) -> String = if rm_punctuation {
        &alphanumeric_only
    } else {
        &|word: String| word.to_string()
    };

    // Reading origin text to a `Vec` of lines represented as `Vec<String>`.
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

    // Creating files to write to.
    let mut encrypted: File = get_file_bound_to(&path_to_file_to_encrypt, "encrypted-");
    let mut decryptor: File = get_file_bound_to(&path_to_file_to_encrypt, "decryptor-for-");

    let action_vecs = ActionVecs::new(approximate_action_set_size, action_word_max_size);
    action_vecs.write_to(&mut decryptor);

    // Encrypting each line one by one.
    for line in origin_lines.iter() {
        encrypt_line(
            line,
            &mut encrypted,
            &mut decryptor,
            &shift_range,
            &encrypted_new_line_length_range,
            &action_vecs,
        );
    }
}

/// Removes all non-alphanumeric characters from the `word`.
///
/// ### Parameters
///
/// `word`: `String` that should be cleared of all non-alphanumeric characters.
///
/// ### Examples
///
/// ```rust, ignore
/// let vegetables = String::from("tomato, onion, cucumber, paprika");
///
/// assert_eq!(alphanumeric_only(vegetables), "tomatoonioncucumberpaprika".to_string());
/// ```
fn alphanumeric_only(word: String) -> String {
    /*
    Possible improvement:
        Add configurability of leaving non-alphanumerical characters located amidst the `word`,
        so that "sea-buckthorn!" turns into "sea-buckthorn", but not into "seabuckthorn".
    */
    word.chars()
        .filter(|chr: &char| chr.is_alphanumeric())
        .collect()
}

/// Encrypts the `line`. Calls [`encrypt_word`] one by one and `'\n'` as well.
///
/// ### Parameters
///
/// `line`: Collection of words contained in a line in the original file.
///
/// `encrypted`: `File` containing encrypted data.
///
/// `decryptor`: `File` containing decryption keys.
///
/// `shift_range`: Maximum and minimum shift of each byte value.
/// E.g. with this equal to `(1, 10)` word `"AAA"` (`[65, 65, 65]` in bytes representation)
/// might look like `"7K>"` (`[55, 75, 62]`).
///
/// `encrypted_new_line_length_range`: Maximum and minimum length of a pseudo-word denoting a new line.
/// E.g. with this equal to (1, 3) `'\n'` might look like `"SHqJY98 KQqrRHqq-6"`
/// as it would be a real word consisting of 2 letters.
///
/// `action_vecs`: `ActionVecs` containing randomly generated action "words".
fn encrypt_line(
    line: &Vec<String>,
    encrypted: &mut File,
    decryptor: &mut File,
    shift_range: &(usize, usize),
    encrypted_new_line_length_range: &(usize, usize),
    action_vecs: &ActionVecs,
) {
    // Encrypting each word one by one.
    for word in line {
        encrypt_word(word, encrypted, decryptor, shift_range, action_vecs);
    }

    // Getting a representation of '\n' as an encrypted pseudo-word.
    let encrypted_new_line: String =
        get_encrypted_new_line(encrypted_new_line_length_range, shift_range, action_vecs);
    write!(encrypted, "{}\n", encrypted_new_line).unwrap();
    write!(decryptor, "0\n").unwrap();
}

/// Encrypts the `word`.
///
/// ### Parameters
///
/// `word`: Word that is encrypted and written to `encrypted`.
///
/// `encrypted`: `File` containing encrypted data.
///
/// `decryptor`: `File` containing decryption keys.
///
/// `shift_range`: Maximum and minimum shift of each byte value.
/// E.g. with this equal to `(1, 10)` word `"AAA"` (`[65, 65, 65]` in bytes representation)
/// might look like `"7K>"` (`[55, 75, 62]`).
///
/// `action_vecs`: `ActionVecs` containing randomly generated action "words".
fn encrypt_word(
    word: &str,
    encrypted: &mut File,
    decryptor: &mut File,
    shift_range: &(usize, usize),
    action_vecs: &ActionVecs,
) {
    let mut rng = rand::thread_rng();

    for byte in word.as_bytes() {
        let (key, action) = (
            rng.gen_range(shift_range.0..shift_range.1) as isize,
            rng.gen_bool(0.5),
        );
        write!(decryptor, "{} ", key).unwrap();

        // The '_' at the beginning is to omit the fact
        // that `_encrypted_byte` is never used before its reassignment in the next `if-else` block.
        let mut _encrypted_byte = EncryptedByte::empty();

        if action {
            _encrypted_byte = EncryptedByte::filled(
                action_vecs.get_random_plus().to_owned(),
                *byte as isize + key,
            );
        } else {
            _encrypted_byte = EncryptedByte::filled(
                action_vecs.get_random_minus().to_owned(),
                *byte as isize - key,
            );
        }

        write!(encrypted, "{}", _encrypted_byte).unwrap();
    }

    write!(encrypted, "\n").unwrap();
    write!(decryptor, "\n").unwrap();
}

/// Returns encrypted notation of `'\n'` combined from so-called units produced by [`get_encrypted_new_line_unit`].
///
/// ### Parameters
///
/// `encrypted_new_line_length_range`: Maximum and minimum length of a pseudo-word denoting a new line.
/// E.g. with this equal to (1, 3) `'\n'` might look like `"SHqJY98 KQqrRHqq-6"`
/// as it would be a real word consisting of 2 letters.
///
/// `shift_range`: Maximum and minimum shift of each byte value.
/// E.g. with this equal to `(1, 10)` word `"AAA"` (`[65, 65, 65]` in bytes representation)
/// might look like `"7K>"` (`[55, 75, 62]`).
///
/// `action_vecs`: `ActionVecs` containing randomly generated action "words".
fn get_encrypted_new_line(
    encrypted_new_line_length_range: &(usize, usize),
    shift_range: &(usize, usize),
    action_vecs: &ActionVecs,
) -> String {
    let mut result: Vec<String> = Vec::new();

    // Determining the finite length of the encrypted `'\n'` notation in "units".
    let finite_encrypted_new_line_length: usize = rand::thread_rng()
        .gen_range(encrypted_new_line_length_range.0..encrypted_new_line_length_range.1);

    // Filling the encrypted `'\n'` notation with "units".
    for _ in 0..finite_encrypted_new_line_length {
        result.push(get_encrypted_new_line_unit(shift_range, action_vecs));
    }

    result.join(" ")
}

/// Returns one unit of encrypted `'\n'`.
///
/// `shift_range`: Maximum and minimum shift of each byte value.
/// E.g. with this equal to `(1, 10)` word `"AAA"` (`[65, 65, 65]` in bytes representation)
/// might look like `"7K>"` (`[55, 75, 62]`).
///
/// `action_vecs`: `ActionVecs` containing randomly generated action "words".
fn get_encrypted_new_line_unit(shift_range: &(usize, usize), action_vecs: &ActionVecs) -> String {
    // TODO: Simplify and solidify the algorithm.
    let mut rng = rand::thread_rng();
    let mut shift_: isize = rng.gen_range(shift_range.0..shift_range.1) as isize;
    shift_ = if rng.gen_bool(0.5) { shift_ } else { -shift_ };

    let new_line_notation = if shift_ > 0 {action_vecs.get_random_plus()} else {action_vecs.get_random_minus()};
    let phantom_number: isize = new_line_notation
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
    fn test_alphanumeric_only() {
        assert_eq!(alphanumeric_only("Hello!".to_string()), "Hello".to_string());
        assert_eq!(
            alphanumeric_only("Hello...".to_string()),
            "Hello".to_string()
        );
    }
}
