// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;

fn main() {
    assert_eq!(p1::get_vowel_count("abracadabra"), 5);
    assert_eq!(
        p2::tribonacci(&[0., 1., 1.], 10),
        vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]
    );
    assert_eq!(
        p3::accum("ZpglnRxqenU"),
        "Z-Pp-Ggg-Llll-Nnnnn-Rrrrrr-Xxxxxxx-Qqqqqqqq-Eeeeeeeee-Nnnnnnnnnn-Uuuuuuuuuuu"
    );
    assert_eq!(p4::find_even_index(&[1, 2, 3, 4, 3, 2, 1]), Some(3));
    assert_eq!(p5::multiple_sum(10), 23);
    assert_eq!(
        p6::create_phone_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 0]),
        "(123) 456-7890"
    );
    assert_eq!(
        p7::disemvowel("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
    assert_eq!(p8::duplicate_encode("(( @"), "))((");
}
