Hello there. The whole program is supposed to process text data of the following format:

```file:
    Foo. Foo foo!
    Bar bar, bar.
    
stored text:
    [["Foo.", "Foo", "foo!"],
     ["Bar", "bar,", "bar."]]```
     
With the given flags `to_lower` and `rm_punctuation` corresponding operations are performed on each word of each line.
I cannot find out how to implement removing of punctuation marks.

```rust
fn process_text(
        path_to_file_to_encrypt: &Path,
        to_lower: bool,
        rm_punctuation: bool,
    ) {
        let origin_lines: Vec<Vec<String>> = fs::read_to_string(&path_to_file_to_process)
            .unwrap()
            .lines()
            .map(|line: &str| line.trim()
                .split(" ")
                .map(if to_lower {|word: &str| word.to_lowercase()} else {|word: &str| word.to_string()})
                .map(if rm_punctuation {cleared} else {|word: &str|word.to_string()}) // type mismatch in function arguments
                                                                                      // expected function signature `fn(String) -> _`
                                                                                          // found function signature `for<'a> fn(&'a str) -> _`
                .collect())
            .collect();
        
        
        // Further processing...
}

// First this function used regular expressions and the trouble came up.
// It remained even after the function has been brought to this form.
fn cleared(word: &str) -> String {
        let mut result = String::from(word);
        let mut n = word.len() - 1;
        
        while word.chars().nth(n).unwrap().is_ascii_punctuation() {
            result.remove(n);
            n -= 1;
        }
        
        result
}
```