struct SmallestPrimeFactor {
    spf: Vec<usize>,
}

impl SmallestPrimeFactor {
    /// Precomputes smallest prime factors for 0..=n in O(n log log n).
    fn new(n: usize) -> Self {
        let mut spf = vec![0; n + 1];
        for p in 2..=n {
            if spf[p] != 0 {
                continue;
            }
            spf[p] = p;
            if p <= n / p {
                for multiple in (p * p..=n).step_by(p) {
                    if spf[multiple] == 0 {
                        spf[multiple] = p;
                    }
                }
            }
        }
        Self { spf }
    }

    /// Returns sorted (prime, exponent) pairs. Requires 1 <= x <= n.
    fn factorize(&self, mut x: usize) -> Vec<(usize, usize)> {
        assert!(x >= 1, "prime factorization requires x >= 1");
        assert!(x < self.spf.len(), "x exceeds the precomputed range");
        let mut factors = Vec::new();
        while x > 1 {
            let p = self.spf[x];
            let mut exponent = 0;
            while x % p == 0 {
                x /= p;
                exponent += 1;
            }
            factors.push((p, exponent));
        }
        factors
    }
}
