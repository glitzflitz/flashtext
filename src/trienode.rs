use std::borrow::Borrow;
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct KeywordTrieNode {
    keyword: Option<String>,
    clean_name: Option<String>,
    children: HashMap<char, KeywordTrieNode>,
}

impl fmt::Display for KeywordTrieNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();
        if let Some(word) = self.get_word() {
            string.push_str(word.as_str());
        }
        if !self.children.is_empty() {
            for (key, value) in self.children.borrow() {
                string.push_str(format!("{}:{} | ", key, value.to_string()).as_str());
            }
        }
        write!(f, "{}\n", string)
    }
}

impl KeywordTrieNode {
    pub fn new(
        keyword: Option<String>,
        clean_name: Option<String>,
        children: HashMap<char, KeywordTrieNode>,
    ) -> KeywordTrieNode {
        KeywordTrieNode {
            keyword: keyword,
            clean_name: clean_name,
            children: children,
        }
    }

    pub fn contains(&self, key: char) -> bool {
        self.children.contains_key(&key)
    }

    pub fn contains_word(&self, word: &str) -> bool {
        match &self.keyword {
            Some(keyword) => keyword.as_str() == word,
            None => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn get(&self, c: char) -> Option<KeywordTrieNode> {
        self.children.get(&c).cloned()
    }

    pub fn get_word(&self) -> Option<String> {
        match self.clean_name.borrow() {
            Some(clean_name) => Some(clean_name.to_string()),
            None => self.keyword.to_owned(),
        }
    }

    pub fn add(
        &mut self,
        word: &str,
        clean_name: &str,
        mut characters: &mut Vec<char>,
    ) -> KeywordTrieNode {
        if characters.is_empty() {
            return KeywordTrieNode::new(
                Some(word.to_string()),
                Some(clean_name.to_string()),
                HashMap::new(),
            );
        }
        while !characters.is_empty() {
            let ch = characters.remove(0);
            let mut node = self
                .get(ch)
                .unwrap_or(KeywordTrieNode::new(None, None, HashMap::new()));
            self.children
                .insert(ch, node.add(word, clean_name, &mut characters));
        }
        self.to_owned()
    }
}
