use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    let target = char_sort(word);
    // for each word in the candidates,
    for candidate in possible_anagrams {
        // if it's an anagram of the target word,
        if target.eq_ignore_ascii_case(&char_sort(candidate)) {
            // add it to the list of matches
           anagrams.insert(*candidate);
        }
    }
    anagrams
}

pub fn char_sort<'a>(word: &'a str) -> String {
    let mut l: Vec<char> = word.chars().collect();
    l.sort_unstable();
    // l.into_iter().collect::<Vec<_>>().as_str()
    l.into_iter().collect::<String>().to_lowercase()
    // s.as_str()
    // let j: &'a str = s.as_str();
    // j
}