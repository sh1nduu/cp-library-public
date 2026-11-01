## Code 1

```rust
let y = cnt_all[&color] - x;
ans += x * y;
*cnt_all.get_mut(&color).unwrap() -= x;
```


## Code 2

```rust
if dist1[u] == diam / 2 && dist2[u] == diam / 2 {
    center = u;
}
```


## Code 3

```text
ans += count_total[k] - count_subtree[k]
count_total[k] += 1
count_subtree[k] += 1
```
