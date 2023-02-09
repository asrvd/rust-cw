pub fn valid_parentheses(s: &str) -> bool {
    if s.is_empty() {
        true
    } else {
        s.starts_with("(")
            && s.ends_with(")")
            && (s
                .chars()
                .map(|c| c.to_string())
                .filter(|c| c == "(")
                .count()
                == s.chars()
                    .map(|c| c.to_string())
                    .filter(|c| c == ")")
                    .count())
    }
}
