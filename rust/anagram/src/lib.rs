use std::collections::{HashSet, HashMap};

fn get_word_count_hashset(word: &str) -> HashMap<char, u8> {
    let mut word_count: HashMap<char, u8> = HashMap::new();
    for char in String::from(word).to_lowercase().chars() {
        *word_count.entry(char).or_insert(0) += 1
    }
    word_count
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut matches: HashSet<&'a str> = HashSet::new();
    let w_hm = get_word_count_hashset(word);
    for possible_anagram in possible_anagrams {
        if String::from(word).to_lowercase() == String::from(*possible_anagram).to_lowercase() { continue; }
        let w2_hm = get_word_count_hashset(possible_anagram);
        if w_hm == w2_hm {
            matches.insert(possible_anagram);
        }
    }
    matches
}