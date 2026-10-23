/// Returns sorted divisors from distinct (prime, exponent) pairs.
/// The represented positive integer must fit in usize. Empty factors represent 1.
fn divisors_from_factors(factors: &[(usize, usize)]) -> Vec<usize> {
    let mut divisors = vec![1];
    for &(p, exponent) in factors {
        let len = divisors.len();
        let mut power = 1;
        for _ in 0..exponent {
            power *= p;
            for i in 0..len {
                divisors.push(divisors[i] * power);
            }
        }
    }
    divisors.sort_unstable();
    divisors
}
