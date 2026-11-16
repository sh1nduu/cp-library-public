## Code 1

```rust
let mut b = s;
while b > 0 {
    let prev = s ^ b;
    if dp[prev][c - 1] != i64::MAX {
        chmin!(dp[s][c], dp[prev][c - 1].max(cost[b]));
    }
    b = (b - 1) & s;
}
```
