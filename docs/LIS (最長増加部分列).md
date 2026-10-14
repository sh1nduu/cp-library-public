## Code 1

```rust
let mut dp = vec![];

for x in a {
    let i = dp.partition_point(|&v| v < x); // lower_bound
    if i == dp.len() {
        dp.push(x);
    } else {
        dp[i] = x;
    }
}

println!("{}", dp.len());
```


## Code 2

```rust
let i = dp.partition_point(|&v| v <= x); // upper_bound
```
