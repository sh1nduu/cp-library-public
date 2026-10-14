## Code 1

```rust
let n = 5;
let graph: Vec<Vec<usize>> = vec![
    vec![1, 2, 3],
    vec![0, 2, 4],
    vec![0, 1, 3, 4],
    vec![2, 4],
    vec![1, 2, 3]
];
let mut dist = vec![None; n];
let mut queue = VecDeque::new();

// 頂点 0 を初期ノードとする
dist[0] = Some(0);
queue.push_back(0);

while let Some(v) = queue.pop_front() {
    for &to in graph[v].iter() {
        if dist[to].is_some() {
            continue;
        }
        dist[to] = dist[v].map(|x| x + 1);
        queue.push_back(to);
    }
}

// 頂点 n - 1 の最短距離 (ただし, -1の場合は到達不可能)
let ans = dist[n-1].unwrap_or(-1);
println!("{ans}")
```
