pub mod actions {
    use rand::seq::SliceRandom;
    use std::collections::HashSet;

    type ActionSet = HashSet<String>;

    const ASCII_LETTERS: &str = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";

    pub fn get_action_sets(set_size: usize, word_size: usize) -> (ActionSet, ActionSet, ActionSet) {
        let action_plus: ActionSet = get_random_set(set_size, word_size);
        let action_minus: ActionSet = get_random_set(set_size, word_size);
        let action_new_line: ActionSet = get_random_set(set_size, word_size);

        // TODO: Implement the difference of each set with two others.

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
