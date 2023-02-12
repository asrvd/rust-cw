use std::collections::HashMap;

pub fn prime_factors(n: i64) -> String {
    let mut prime_factors = HashMap::new();
    let mut n = n;

    while n % 2 == 0 {
        let count = prime_factors.entry(2).or_insert(0);
        *count += 1;
        n = n / 2;
    }

    for i in (3..(n as f64).sqrt() as i64 + 1).step_by(2) {
        while n % i == 0 {
            let count = prime_factors.entry(i).or_insert(0);
            *count += 1;
            n = n / i;
        }
    }

    if n > 2 {
        let count = prime_factors.entry(n).or_insert(0);
        *count += 1;
    }

    if prime_factors.is_empty() {
        return format!("({})", n);
    }

    let mut prime_factors: Vec<_> = prime_factors.into_iter().collect();
    prime_factors.sort_by(|a, b| a.0.cmp(&b.0));

    println!("{:?}", prime_factors);

    format!(
        "{}",
        prime_factors
            .iter()
            .map(|(k, v)| {
                if *v == 1 {
                    format!("({})", k)
                } else {
                    format!("({}**{})", k, v)
                }
            })
            .collect::<Vec<String>>()
            .join("")
    )
}
