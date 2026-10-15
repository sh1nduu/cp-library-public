## Code 1

```rust
struct MaxUpdate;

impl MapMonoid for MaxUpdate {
    type M = Max<i64>;
    type F = Option<i64>;

    fn identity_map() -> Self::F {
        None
    }

    fn mapping(f: &Self::F, x: &i64) -> i64 {
        f.unwrap_or(*x)
    }

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        f.or(*g)
    }
}
```


## Code 2

```text
Some(5) → Some(8) = Some(8)
Some(5) → None    = Some(5)
None    → Some(8) = Some(8)
```


## Code 3

```rust
f.or(*g)
```


## Code 4

```rust
let x = seg.prod(l..=r);
seg.apply_range(l..=r, Some(x + 1));
println!("{}", x + 1);
```
