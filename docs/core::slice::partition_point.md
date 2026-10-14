## Code 1

```rust
let i = v.partition_point(|&x| x < k);
```


## Code 2

```rust
let i = v.partition_point(|&x| x <= k);
```


## Code 3

```rust
let i = v.partition_point(|&x| x < k);
let count = v.len() - i;
```


## Code 4

```rust
let i = v.partition_point(|&x| x <= k);
let count = v.len() - i;
```
