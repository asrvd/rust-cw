// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;

mod p1;
mod p10;
mod p11;
mod p12;
mod p13;
mod p14;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;

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
    assert_eq!(p9::split_strings("abcdefg"), ["ab", "cd", "ef", "g_"]);
    assert_eq!(p11::bouncing_ball(3.0, 0.66, 1.5), 3);
    assert_eq!(p12::valid_parentheses("((()))()"), true);
    assert_eq!(p13::prime_factors(7775460), "(2**2)(3**3)(5)(7)(11**2)(17)");
    assert_eq!(p14::sq_in_rect(5, 3), Some(vec![3, 2, 1, 1]));
}
