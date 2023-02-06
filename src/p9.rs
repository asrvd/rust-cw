pub fn split_strings(s: &str) -> Vec<String> {
    s.chars()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .chunks(2)
        .collect::<Vec<_>>()
        .iter_mut()
        .map(|chunk| {
            if chunk.len() > 1 {
                chunk.join("")
            } else {
                chunk.join("") + "_"
            }
        })
        .collect::<Vec<String>>()
}
