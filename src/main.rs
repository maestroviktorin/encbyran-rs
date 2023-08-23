/*
Work in Progress.
*/
use std::path::Path;

use encbyran::*;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Command,
}

#[derive(Clone, Subcommand)]
enum Command {
    /// Encrypt a file and receive a Decryptor.
    Encrypt {
        /// Location of the file to be encrypted.
        #[arg(short = 'f', long)]
        file: String,

        /// Whether to bring all the letters to the lowercase or not.
        #[arg(long)]
        to_lower: bool,

        /// Whether to omit all the punctuation characters or not.
        #[arg(long)]
        rm_punctuation: bool,

        /// Maximum and minimum shift of each byte value in the form `<min>,<max>`.
        #[arg(long, value_parser = utils::range_parser, default_value = "1,200")]
        shift_range: utils::UsizeRangeTupleContainer,

        /// Maximum and minimum length of a pseudo-word denoting a new line in the form `<min>,<max>`.
        #[arg(long, value_parser = utils::range_parser, default_value = "1,15")]
        encrypted_new_line_length_range: utils::UsizeRangeTupleContainer,

        /// Approximate size of a set of special randomly generated "words"
        /// indicating the direction of the byte value shift.
        #[arg(long, default_value_t = 10)]
        approximate_action_set_size: usize,

        /// Maximum size of a special randomly generated "word"
        /// indicating the direction of the byte value shift.
        #[arg(long, default_value_t = 10)]
        action_word_max_size: usize,
    },

    /// Decrypt an Encrypted file using a Decryptor for it.
    Decrypt {
        /// Location of the Encrypted File to be decrypted.
        #[arg(short = 'f', long)]
        file: String,

        /// Location of Decryptor to the Encrypted File.
        #[arg(short = 'd', long)]
        decryptor: String,
    },
}

fn main() {
    let cli = CLI::parse();

    match cli.command {
        Command::Encrypt {
            file,
            to_lower,
            rm_punctuation,
            shift_range,
            encrypted_new_line_length_range,
            approximate_action_set_size,
            action_word_max_size,
        } => cryptograph::cryptograph(
            Path::new(&file),
            to_lower,
            rm_punctuation,
            shift_range.0,                     // Obtaining from the container struct.
            encrypted_new_line_length_range.0, // Obtaining from the container struct.
            approximate_action_set_size,
            action_word_max_size,
        ),

        Command::Decrypt { file, decryptor } => {
            decryptograph::decryptograph(Path::new(&file), Path::new(&decryptor))
        }
    }
}
