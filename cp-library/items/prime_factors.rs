/// Returns (prime, exponent) pairs in ascending prime order. Requires x >= 1.
fn prime_factors(mut x: usize) -> Vec<(usize, usize)> {
    assert!(x >= 1, "prime factorization requires x >= 1");
    let mut factors = Vec::new();
    let mut p = 2;
    while p <= x / p {
        let mut exponent = 0;
        while x % p == 0 {
            x /= p;
            exponent += 1;
        }
        if exponent > 0 {
            factors.push((p, exponent));
        }
        p += 1;
    }
    if x > 1 {
        factors.push((x, 1));
    }
    factors
}
