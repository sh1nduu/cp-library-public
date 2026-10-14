## Code 1

```
i → i - lowbit(i) → i - lowbit(i) - lowbit(i-lowbit(i)) → ...
```


## Code 2

```
i → i + lowbit(i) → i + lowbit(i+lowbit(i)) → ...
```


## Code 3

```rust
#[derive(Debug)]
#[allow(clippy::upper_case_acronyms)]
struct BIT {
    /// 1-indexed
    bit: Vec<i64>,
}

impl BIT {
    fn new(n: usize) -> Self {
        Self { bit: vec![0; n + 1] }
    }

    fn sum(&self, mut i: usize) -> i64 {
        let mut s = 0;
        while i > 0 {
            s += self.bit[i];
            i -= i & (!i + 1); // lowbit
        }
        s
    }

    fn add(&mut self, mut i: usize, x: i64) {
        let n = self.bit.len() - 1;
        while i <= n {
            self.bit[i] += x;
            i += i & (!i + 1);
        }
    }
}
```
