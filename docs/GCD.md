## Code 1

```rust
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}
```


## Code 2

  ```text
  g = gcd(a1, a2, ..., aN)
    = gcd(0, a1, a2, ..., aN)
  ```


## Code 3

```rust
let mut g = 0;
for &x in &a {
    g = gcd(g, x);
}
```
