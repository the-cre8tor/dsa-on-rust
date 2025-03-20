use std::collections::HashMap;

// Space & Time Complexity:
//   - Time: O(n) - single pass through each array
//   - Space: O(n) - uses a HashMap to store squares
pub fn same_linked(value_one: &[u32], value_two: &[u32]) -> bool {
    if value_one.len() != value_two.len() {
        return false;
    }

    let mut map_one: HashMap<u32, u32> = HashMap::new();
    let mut map_two: HashMap<u32, u32> = HashMap::new();

    for item in value_one {
        let squared = item.pow(2);
        *map_one.entry(squared).or_insert(0) += 1;
    }

    for item in value_two {
        *map_two.entry(*item).or_insert(0) += 1;
    }

    for item in map_one.keys() {
        if map_two.get(item).is_none() {
            return false;
        }

        if map_one[item] != map_two[item] {
            return false;
        }
    }

    true
}

// Anagrams
// Given two strings, write a function to determine if the
// second string is an anagram of the first.

pub fn validate_anagram(word_one: &str, word_two: &str) -> bool {
    if word_one.len() != word_one.len() {
        return false;
    }

    let mut map_one: HashMap<char, u8> = HashMap::new();
    let mut map_two: HashMap<char, u8> = HashMap::new();

    for item in word_one.to_lowercase().chars() {
        *map_one.entry(item).or_insert(0) += 1;
    }

    for item in word_two.to_lowercase().chars() {
        *map_two.entry(item).or_insert(0) += 1;
    }

    for item in map_one.keys() {
        if !map_two.contains_key(item) {
            return false;
        }

        if map_one[item] != map_two[item] {
            return false;
        }
    }

    true
}

pub fn anagram_lite(word_one: &str, word_two: &str) -> bool {
    if word_one.len() != word_one.len() {
        return false;
    }

    let mut lookup: HashMap<char, u8> = HashMap::new();

    for item in word_one.chars() {
        *lookup.entry(item).or_insert(0) += 1;
    }

    for item in word_two.chars() {
        if !lookup.contains_key(&item) {
            return false;
        }

        *lookup.entry(item).or_insert(0) -= 1;
    }

    true
}
