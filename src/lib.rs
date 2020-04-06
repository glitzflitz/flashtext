//! # Flashtext
//! Rust implementation of [flashtext](https://arxiv.org/abs/1711.00046) algorithm for
//! searching and replacing keywords in given text
//!
//! # Examples
//!
//! # Find keyword/sentence
//! ```
//! use flashtext::KeywordProcessor;
//! let mut keywordprocessor = KeywordProcessor::new(false);
//! keywordprocessor.add_keyword("apple");
//! println!("{:?}", keywordprocessor.find_keywords("An apple fell from the tree"));
//!```
//!
//!# Replace keyword/sentence
//!```
//!use flashtext::KeywordProcessor;
//!let mut keywordprocessor = KeywordProcessor::new(false);
//!keywordprocessor.add_keywords("dancing", "reading");
//!println!("{}", keywordprocessor.replace_keywords("She likes dancing"));

pub mod trienode;
use crate::trienode::KeywordTrieNode;
use std::collections::{HashMap, HashSet};

/// Struct which represents the state
pub struct KeywordProcessor {
    case_sensitive: bool,
    root_node: KeywordTrieNode,
}

impl KeywordProcessor {
    /// Takes a bool for case sensitivity and returns instance of a KeywordProcessor
    ///
    ///# Arguments
    ///
    ///* `case_sensitive` - A bool which indicates case sensitivity
    ///
    ///# Example
    ///
    ///```
    /// use flashtext::KeywordProcessor;
    /// let mut keywordprocessor = KeywordProcessor::new(false);
    /// ```
    pub fn new(case_sensitive: bool) -> KeywordProcessor {
        KeywordProcessor {
            case_sensitive,
            root_node: KeywordTrieNode::new(None, None, HashMap::new()),
        }
    }

    /// Takes a keyword/sentence to search
    ///
    /// # Arguments
    ///
    /// * `word` - A string slice that holds a keyword/sentence
    ///
    /// # Example
    ///
    /// ```
    /// use flashtext::KeywordProcessor;
    /// let mut keywordprocessor = KeywordProcessor::new(false);
    /// keywordprocessor.add_keyword("keyword");
    /// ```
    pub fn add_keyword(&mut self, word: &str) {
        let input_word = if self.case_sensitive {
            word.to_ascii_lowercase()
        } else {
            word.to_string()
        };
        self.add_keywords(input_word.as_str(), input_word.as_str());
    }

    /// Takes a keyword/sentence to replace another keyword/sentence
    ///
    /// # Arguments
    ///
    /// * `word` - The keyword/sentence to replace
    /// * `clean_name` - The keyword/sentence to replace with
    ///
    /// # Example
    ///
    /// ```
    /// use flashtext::KeywordProcessor;
    /// let mut keywordprocessor = KeywordProcessor::new(false);
    /// keywordprocessor.add_keywords("keyword", "new_keyword");
    /// ```
    pub fn add_keywords(&mut self, word: &str, clean_name: &str) {
        let (input_word, input_clean_name) = if self.case_sensitive {
            (word.to_ascii_lowercase(), clean_name.to_ascii_lowercase())
        } else {
            (word.to_string(), clean_name.to_string())
        };
        self.root_node.add(
            input_word.as_str(),
            input_clean_name.as_str(),
            &mut input_word.chars().collect::<Vec<char>>(),
        );
    }

    /// Returns a `HashSet` of String if a given keyword/sentence exists in the input string
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice which holds the input string in which keyword are to be searched
    /// or replaced
    ///
    /// # Example
    ///
    /// ```
    /// use flashtext::KeywordProcessor;
    /// use std::collections::HashSet;
    /// let mut keywordprocessor = KeywordProcessor::new(false);
    /// keywordprocessor.add_keyword("programming");
    /// let result = keywordprocessor.find_keywords("Rust is a systems programming language");
    /// let mut expected = HashSet::new();
    /// expected.insert("programming".to_string());
    /// assert_eq!(result, expected);
    pub fn find_keywords(&self, input: &str) -> HashSet<String> {
        let mut keywords: HashSet<String> = HashSet::new();
        let mut current_node = self.root_node.to_owned();
        input.chars().for_each(|c| {
            let character = if self.case_sensitive {
                c.to_ascii_lowercase()
            } else {
                c
            };
            match current_node.get(character) {
                Some(node) => {
                    current_node = node.to_owned();
                    if let Some(word) = current_node.get_word() {
                        keywords.insert(word);
                    }
                }
                None => {
                    current_node = self.root_node.to_owned();
                }
            }
        });
        keywords
    }

    /// Returns a String after replacing keywords in input specified by `add_keywords`
    ///
    /// # Arguments
    /// * `input_str` - A string slice which holds input string in which keywords are needed to be
    /// replaced
    ///
    /// # Example
    ///
    /// ```
    /// use flashtext::KeywordProcessor;
    /// let mut keywordprocessor = KeywordProcessor::new(false);
    /// keywordprocessor.add_keywords("Alice", "Bob");
    /// let result = keywordprocessor.replace_keywords("Alice likes to read a book");
    /// let expected = String::from("Bob likes to read a book");
    /// assert_eq!(result, expected);
    /// ```
    pub fn replace_keywords(&mut self, input: &str) -> String {
        let mut output = String::new();
        let mut buffer = String::new();
        let mut current_node = self.root_node.to_owned();
        input.chars().for_each(|c| {
            let character = if self.case_sensitive {
                c.to_ascii_lowercase()
            } else {
                c
            };
            let node = current_node.get(character);
            if let Some(value) = node {
                current_node = value;
                match current_node.get_word() {
                    Some(name) => {
                        buffer.clear();
                        buffer.push_str(name.as_str());
                    }
                    None => {
                        buffer.push(c);
                    }
                }
            } else {
                output.push_str(buffer.as_str());
                output.push(c);
                buffer.clear();
                current_node = self.root_node.to_owned();
            }
        });
        if !buffer.is_empty() {
            output.push_str(buffer.as_str());
        }
        output
    }
}
