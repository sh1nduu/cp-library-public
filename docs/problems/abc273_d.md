## Code 1

```rust
fn move_1d_negative(pos: usize, step: usize, walls: &[usize]) -> usize {
    let i = walls.partition_point(|&x| x < pos).saturating_sub(1);
    pos.saturating_sub(step).max(walls[i] + 1)
}
```


## Code 2

```rust
fn move_1d_positive(pos: usize, step: usize, walls: &[usize]) -> usize {
    let i = walls.partition_point(|&x| x <= pos);
    pos.saturating_add(step).min(walls[i] - 1)
}
```
