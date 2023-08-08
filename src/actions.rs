pub mod actions {
    use rand::seq::SliceRandom;
    use std::collections::HashSet;

    type ActionSet = HashSet<String>;

    const ASCII_LETTERS: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

    pub fn get_action_sets(set_size: usize, word_size: usize) -> (ActionSet, ActionSet, ActionSet) {
        let mut action_plus: ActionSet = get_random_set(set_size, word_size);
        let mut action_minus: ActionSet = get_random_set(set_size, word_size);
        let mut action_new_line: ActionSet = get_random_set(set_size, word_size);

        // Solution obtained from https://stackoverflow.com/questions/76860337/compound-hashset-operations-in-rust
        (action_plus, action_minus, action_new_line) = (
            &(&action_plus - &action_minus) - &action_new_line,
            &(&action_minus - &action_plus) - &action_new_line,
            &(&action_new_line - &action_plus) - &action_minus,
        );

        (action_plus, action_minus, action_new_line)
    }

    fn get_random_set(set_size: usize, word_size: usize) -> ActionSet {
        let mut result: ActionSet = HashSet::new();

        for _ in 0..set_size {
            result.insert(get_random_word(word_size));
        }

        result
    }

    fn get_random_word(word_size: usize) -> String {
        // https://docs.rs/rand/latest/rand/seq/index.html
        let mut result: String = String::new();
        let mut rng = rand::thread_rng();

        for _ in 1..word_size {
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
}
