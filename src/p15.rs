fn encode(c: char) -> char {
    let o: u32 = c.into();

    let e: char = if c.is_uppercase() {
        if o + 13 > 90 {
            (o + 13 - 26).try_into().unwrap()
        } else {
            (o + 13).try_into().unwrap()
        }
    } else {
        if o + 13 > 122 {
            (o + 13 - 26).try_into().unwrap()
        } else {
            (o + 13).try_into().unwrap()
        }
    };

    e
}

pub fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| if c.is_alphabetic() { encode(c) } else { c })
        .collect()
}
