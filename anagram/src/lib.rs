use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let lower_word = word.to_lowercase();
    let target = char_sort(&lower_word);
    // for each word in the candidates,
    for candidate in possible_anagrams {
        let lower_candidate = candidate.to_lowercase();
        // words are not anagrams of themselves
        if lower_word == lower_candidate {
            continue;
        }
        // if it's an anagram of the target word,
        if target == char_sort(&lower_candidate) {
            // add it to the list of matches
            anagrams.insert(*candidate);
        }
    }
    anagrams
}

pub fn char_sort<'a>(word: &'a String) -> String {
    let mut s: Vec<char> = word.chars().collect();
    s.sort_unstable();
    s.into_iter().collect::<String>()
}
