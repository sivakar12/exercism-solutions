use std::collections::{HashSet};

fn get_sorted_lowercase(word: &str) -> Vec<char> {
    let mut word_lowercase = word.to_lowercase().chars().collect::<Vec<char>>();
    word_lowercase.sort_unstable();
    word_lowercase
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lowercase = word.to_lowercase();
    let word_sorted_lowercase = get_sorted_lowercase(word);
    possible_anagrams
        .iter()
        .filter(|possible_anagram| {
            possible_anagram.to_lowercase() != word.to_lowercase() &&
            word_lowercase.len() == possible_anagram.to_lowercase().len() &&
            word_sorted_lowercase == get_sorted_lowercase(possible_anagram)
        })
        .copied()
        .collect()
}