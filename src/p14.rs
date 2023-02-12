use std::cmp;

pub fn sq_in_rect(lng: i32, wdth: i32) -> Option<Vec<i32>> {
    let (mut lng, mut wdth) = (lng, wdth);

    let mut smaller = cmp::min(lng, wdth);

    let mut squares = Vec::new();

    while smaller > 0 {
        squares.push(smaller);

        if lng > wdth {
            lng = lng - smaller;
        } else {
            wdth = wdth - smaller;
        };

        smaller = cmp::min(lng, wdth)
    }

    if squares.len() == 1 {
        // if the given input is square
        None
    } else {
        Some(squares)
    }
}
