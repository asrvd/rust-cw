pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h > 0.0 && window < h && bounce > 0.0 && bounce < 1.0 {
        let mut height = h * bounce;
        let mut count = 0;

        while height > window {
            count += 1;
            height = height * bounce
        }

        count * 2 + 1 // 1 while it falls down for first time and *2 cus ball is visible 2 times each time it bounces
    } else {
        -1
    }
}
