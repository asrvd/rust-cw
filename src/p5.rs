// sum of all multiples of 3 or 5 in the range [1, num)

pub fn multiple_sum(num: i32) -> i32 {
    (3..num).filter(|n| *n % 3 == 0 || *n % 5 == 0).sum()
}
