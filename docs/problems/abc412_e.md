## Code 1

```rust
let mut used = vec![false; r - l + 1];
let mut ans = 1_usize;

for p in primes {
    for mut x in ((l / p + 1) * p..=r).step_by(p) {
        if used[x - l] {
            continue;
        }
        used[x - l] = true;

        while x % p == 0 {
            x /= p;
        }

        if x == 1 {
            ans += 1;
        }
    }
}

// 未処理なら素数
for x in l + 1..=r {
    if !used[x - l] {
        ans += 1;
    }
}
```
