pub fn accum(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| c.to_string().to_uppercase() + (&c.to_lowercase().to_string().repeat(i)))
        .collect::<Vec<String>>()
        .join("-")
}
