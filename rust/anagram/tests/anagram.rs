use std::collections::HashSet;

use anagram::anagrams_for;

#[test]
fn no_matches() {
    assert_eq!(anagrams_for("diaper", &["hello", "world", "zombies", "pants"]), HashSet::new());
}

#[test]
#[ignore]
fn detects_simple_anagram() {
    let expected = HashSet::from(["tan"]);
    assert_eq!(anagrams_for("ant", &["tan", "stand", "at"]), expected);
}

#[test]
#[ignore]
fn does_not_return_the_input_word_itself() {
    let expected = HashSet::from(["inlets"]);
    assert_eq!(anagrams_for("listen", &["enlists", "google", "inlets", "banana", "listen"]), expected);
}

#[test]
#[ignore]
fn is_case_insensitive() {
    let expected = HashSet::from(["Carthorse"]);
    assert_eq!(anagrams_for("Orchestra", &["cashregister", "Carthorse", "radishes"]), expected);
}

