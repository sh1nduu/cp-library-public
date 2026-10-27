## Code 1

```rust
for (l, r, v) in lrv {
    let seg: Segtree<Max<_>> = dp.iter().copied().collect();
    let mut ndp = dp.clone();
    for j in l..=w {
        let best = seg.prod(j.saturating_sub(r)..=j - l);
        if best != ninf {
            chmax!(ndp[j], best + v);
        }
    }
    dp = ndp;
}
```
