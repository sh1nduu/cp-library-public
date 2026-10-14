## Code 1

```rust
let mut used = vec![false; max_a + 1];
let mut ng = vec![false; m + 1];

for &a in &a_list {
    for (p, _) in prime_factors(a) {
        if !used[p] {
            used[p] = true;
            let mut x = p;
            while x <= m {
                ng[x] = true;
                x += p;
            }
        }
    }
}
```
