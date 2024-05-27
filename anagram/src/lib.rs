use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a[&'a str]) -> HashSet<&'a str> {
    let mut anagrams = HashSet::new();
    // for each word in the candidates,
    for candidate in possible_anagrams {
    //   if it's an anagram of the target word,
    //     add it to the list of matches
           // anagrams.insert(*candidate);
    }
    anagrams
}
