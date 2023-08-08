# Answer

```rust
let transform_once: &dyn Fn(&str) -> String = if to_lower {&|word: &str| word.to_lowercase()} else {&|word: &str| word.to_string()};
let transform_twice: &dyn Fn(String) -> String = if rm_punctuation {&cleared} else {&|word: String| word.to_string()};
let origin_lines: Vec<Vec<String>> = fs::read_to_string(&path_to_file_to_encrypt)
    .unwrap()
    .lines()
    .map(|line: &str| line.trim()
        .split(" ")
        .map(transform_once)
        .map(transform_twice)
        ...)
```
