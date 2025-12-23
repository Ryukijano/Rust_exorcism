use std::collections::HashMap;

use word_count::word_count;

#[test]
fn counts_an_empty_string() {
    assert_eq!(word_count(""), HashMap::new());
}

#[test]
#[ignore]
fn counts_one_word() {
    let expected = HashMap::from([("hello".to_string(), 1)]);
    assert_eq!(word_count("hello"), expected);
}

#[test]
#[ignore]
fn counts_multiple_words() {
    let expected = HashMap::from([
        ("one".to_string(), 1),
        ("fish".to_string(), 2),
        ("two".to_string(), 1),
        ("red".to_string(), 1),
        ("blue".to_string(), 1),
    ]);
    assert_eq!(word_count("one fish two fish red fish blue fish"), expected);
}

#[test]
#[ignore]
fn is_case_insensitive() {
    let expected = HashMap::from([("rust".to_string(), 3)]);
    assert_eq!(word_count("Rust rUsT RUST"), expected);
}

#[test]
#[ignore]
fn ignores_punctuation() {
    let expected = HashMap::from([("car".to_string(), 1), ("carpet".to_string(), 1)]);
    assert_eq!(word_count("car, carpet."), expected);
}

#[test]
#[ignore]
fn keeps_apostrophes_inside_words() {
    let expected = HashMap::from([("don't".to_string(), 2), ("stop".to_string(), 1)]);
    assert_eq!(word_count("Don't stop, don't."), expected);
}

