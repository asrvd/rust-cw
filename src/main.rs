// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

mod p1;
mod p2;

fn main() {
    assert_eq!(p1::get_vowel_count("abracadabra"), 5);
    assert_eq!(
        p2::tribonacci(&[0., 1., 1.], 10),
        vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]
    );
}
