pub fn duplicate_encode(word: &str) -> String {
    word.to_lowercase()
        .chars()
        .map(|c| {
            if word.to_lowercase().matches(c).count() > 1 {
                ")"
            } else {
                "("
            }
        })
        .collect()
}
