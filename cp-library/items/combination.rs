#[derive(Debug)]
struct Combination<T> {
    fact: Vec<T>,
    fact_inv: Vec<T>,
}

impl<M: ac_library::Modulus> Combination<ac_library::StaticModInt<M>> {
    fn new(n: usize) -> Self {
        assert!(
            M::HINT_VALUE_IS_PRIME,
            "Combination requires a prime modulus"
        );

        let modulus = ac_library::StaticModInt::<M>::modulus() as usize;
        assert!(n < modulus, "Combination requires n < modulus");

        let mut fact = vec![ac_library::StaticModInt::<M>::new(1); n + 1];
        let mut inv = vec![ac_library::StaticModInt::<M>::new(1); n + 1];
        let mut fact_inv = vec![ac_library::StaticModInt::<M>::new(1); n + 1];

        for i in 2..=n {
            fact[i] = fact[i - 1] * i;
            inv[i] = -inv[modulus % i] * (modulus / i);
            fact_inv[i] = fact_inv[i - 1] * inv[i];
        }

        Self { fact, fact_inv }
    }

    fn calc(&self, n: usize, k: usize) -> ac_library::StaticModInt<M> {
        assert!(n < self.fact.len(), "n exceeds the precomputed range");
        if k > n {
            return ac_library::StaticModInt::<M>::new(0);
        }
        self.fact[n] * self.fact_inv[k] * self.fact_inv[n - k]
    }
}
