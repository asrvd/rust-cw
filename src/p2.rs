// tribonacci sequence 

pub fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let mut signature = signature.to_vec();
    let mut result = Vec::new();

    for _ in 0..n {
        result.push(signature[0]);

        let next = signature[0] + signature[1] + signature[2];

        signature[0] = signature[1];
        signature[1] = signature[2];
        signature[2] = next;
    }

    result
}
