pub fn find_even_index(arr: &[i32]) -> Option<usize> {
    arr.iter()
        .enumerate()
        .filter(|(i, n)| {
            &arr[0..*i].iter().sum::<i32>() | 0 == &arr[*i + 1..arr.len()].iter().sum::<i32>() | 0
        })
        .collect::<Vec<_>>()
        .iter()
        .map(|c| c.0)
        .into_iter()
        .min()
}
