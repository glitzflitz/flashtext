use flashtext::KeywordProcessor;
use std::collections::HashSet;

#[test]
fn test_find_keywords() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keyword("apple");
    let result = keywordprocessor.find_keywords("An apple fell from the tree");
    let mut expected = HashSet::new();
    expected.insert("apple".to_string());
    assert_eq!(
        result, expected,
        "Could not find correct keyword. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn skip_incomplete() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keyword("pineapple");
    let result = keywordprocessor.find_keywords("He ate an apple");
    let expected = HashSet::new();
    assert_eq!(
        result, expected,
        "Failed to skip incomplete word at the end of input. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn find_at_begining() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keyword("Rust");
    let result = keywordprocessor.find_keywords("Rust is great programming language");
    let mut expected = HashSet::new();
    expected.insert("Rust".to_string());
    assert_eq!(
        result, expected,
        "Failed to find word at the begining of input. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn find_multiple_keywords() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keyword("Rust");
    keywordprocessor.add_keyword("Python");
    let result = keywordprocessor.find_keywords("Rust and Python are great programming languages");
    let mut expected = HashSet::new();
    expected.insert("Rust".to_string());
    expected.insert("Python".to_string());
    assert_eq!(
        result, expected,
        "Failed to find words one or more in the input. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn test_overlapping_keywords() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keyword("Rust");
    keywordprocessor.add_keyword("Rustaceans");
    let result = keywordprocessor.find_keywords("Rustaceans are people who use Rust");
    let mut expected = HashSet::new();
    expected.insert("Rust".to_string());
    expected.insert("Rustaceans".to_string());
    assert_eq!(
        result, expected,
        "Failed to find overlapping keywords in the input. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn test_case_sensitivity() {
    let mut keywordprocessor = KeywordProcessor::new(true);
    keywordprocessor.add_keyword("Rust");
    let result = keywordprocessor.find_keywords("rust is great programming language");
    let mut expected = HashSet::new();
    expected.insert("rust".to_string());
    assert_eq!(
        result, expected,
        "Failed to find case sensitive keyword in the input. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn test_replace_at_end() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keywords("C++", "Rust");
    let result = keywordprocessor.replace_keywords("I like C++");
    let expected = String::from("I like Rust");
    assert_eq!(
        result, expected,
        "Failed to replace word at end of the sentence. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn test_skip_replacing_incomplete_words_at_end() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keywords("pineapple", "mango");
    let result = keywordprocessor.replace_keywords("He ate an apple");
    let expected = String::from("He ate an apple");
    assert_eq!(expected, result, "Failed to skip replacing incomplete word at the end of the input. Expected:{:?} returned:{:?}", expected, result);
}

#[test]
fn test_replace_at_begining() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keywords("C++", "Rust");
    let result = keywordprocessor.replace_keywords("C++ is a great systems programming language");
    let expected = String::from("Rust is a great systems programming language");
    assert_eq!(
        expected, result,
        "Failed to replace word at the begining of the input. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn test_replace_multiple() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keywords("C++", "Rust");
    keywordprocessor.add_keywords("Javascript", "Python");
    let result = keywordprocessor.replace_keywords(
        "C++ is a systems programming language while Javascript is scripting language",
    );
    let expected =
        String::from("Rust is a systems programming language while Python is scripting language");
    assert_eq!(
        expected, result,
        "Failed to replace multiple keywords in the input. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn test_replace_with_sentence() {
    let mut keywordprocessor = KeywordProcessor::new(false);
    keywordprocessor.add_keywords("Java", "Rust");
    keywordprocessor.add_keywords("object oriented", "systems programming");
    let result = keywordprocessor.replace_keywords("Java is a object oriented language");
    let expected = String::from("Rust is a systems programming language");
    assert_eq!(
        expected, result,
        "Failed to replace keyword by a sentence. Expected:{:?} returned:{:?}",
        expected, result
    );
}

#[test]
fn test_replace_with_case_sensitive() {
    let mut keywordprocessor = KeywordProcessor::new(true);
    keywordprocessor.add_keywords("APPLE", "banana smoothie");
    let result = keywordprocessor.replace_keywords("I like apple");
    let expected = String::from("I like banana smoothie");
    assert_eq!(
        expected, result,
        "Failed to replace case sensitive keyword. Expected:{:?} returned:{:?}",
        expected, result
    );
}
