# flashtext
[![Crates.io](https://img.shields.io/crates/v/flashtext)]((https://crates.io/crates/flashtext))
[![Documentation](https://docs.rs/flashtext/badge.svg)](https://docs.rs/flashtext/)
[![Build Status](https://travis-ci.org/glitzflitz/flashtext.svg?branch=master)](https://travis-ci.org/glitzflitz/flashtext)
Rust implementation of flashtext algorithm to search and replace keywords in given text

# Usage
Add this to your Cargo.toml
```
[dependencies]
flashtext = "0.1.0"
```
## Documentation


## Examples

### Find keyword/sentence
```rust
use flashtext::KeywordProcessor;
let mut keywordprocessor = KeywordProcessor::new(false);
keywordprocessor.add_keyword("apple");
println!("{:?}", keywordprocessor.find_keywords("An apple fell from the tree");
```

### Replace keyword/sentence
```rust
use flashtext::KeywordProcessor;
let mut keywordprocessor = KeywordProcessor::new(false);
keywordprocessor.add_keywords("dancing", "reading");
println!("{}", keywordprocessor.replace_keywords("She likes dancing"));
```

## TODO

- [x] Add tests
- [x] Add Benchmarks
- [x] Setup CI

## Note
This experimental implementation is for learning purposes. For better performance and more features you should use rust's regex engine.
