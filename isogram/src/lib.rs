use std::collections::HashMap;

pub fn check(candidate: &str) -> bool {
    if candidate.is_empty() { return false }

    candidate.split(|ch: char| ch == '-' || ch.is_whitespace()).all( |word| {
        let mut frequency = HashMap::with_capacity(word.len());
        word.chars().for_each( |ch| (*(frequency.entry(ch.to_lowercase().to_string())).or_insert(0)) += 1);
        frequency.values().all(|num| *num == 1)
    })
}
