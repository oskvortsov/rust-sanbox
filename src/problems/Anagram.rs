// https://exercism.org/tracks/rust/exercises/anagram

// topics #[HashMap, HashSet]

use std::collections::{HashMap, HashSet};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_map = count_word_map(&word_lower);

    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();

            if candidate_lower != word_lower {
                let candidate_map = count_word_map(&candidate_lower);
                return candidate_map.eq(&word_map)
            }

            false
        })
        .copied()
        .collect()
}

fn count_word_map(word: &str) -> HashMap<char, i32> {
    let mut word_map: HashMap<char, i32> = HashMap::new();

    for c in word.chars() {
        word_map.insert(c,  word_map.get(&c).unwrap_or(&0) + 1);
    }

    word_map
}