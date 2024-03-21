use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut vectorized_word: Vec<char> = String::from(word)
        .to_lowercase()
        .chars()
        .into_iter()
        .collect();
    vectorized_word.sort();

    let mut anagrams = HashSet::new();

    for possible_anagram in possible_anagrams {
        if word.to_lowercase() == *possible_anagram.to_lowercase() {
            continue;
        }

        if is_anagram(&vectorized_word, possible_anagram) {
            anagrams.insert(*possible_anagram);
        }
    }

    anagrams
}

fn is_anagram<'b>(word: &Vec<char>, possible_anagram: &'b str) -> bool {
    // ideally we would only want to do this once
    let mut possible_anagram: Vec<char> = String::from(possible_anagram)
        .to_lowercase()
        .chars()
        .into_iter()
        .collect();

    possible_anagram.sort();

    *word == possible_anagram
}
