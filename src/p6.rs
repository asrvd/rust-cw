// create phone number from array of 10 digits

pub fn create_phone_number(numbers: &[u8]) -> String {
    format!(
        "({}) {}-{}",
        numbers[0..3]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>(),
        numbers[3..6]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>(),
        numbers[6..10]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>()
    )
}

// using unstable* chunks method
//
// pub fn create_phone_number(numbers: &[u8]) -> String {
//     let mut chunks = numbers.as_chunks::<3>();

//     chunks
//         .0
//         .iter()
//         .map(|nums| nums.iter().map(|n| n.to_string()).collect::<String>())
//         .collect::<String>()
//         + &chunks.1.iter().map(|n| n.to_string()).collect::<String>()
// }
