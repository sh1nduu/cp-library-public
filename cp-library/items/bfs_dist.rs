fn bfs_dist(start: usize, graph: &[Vec<usize>]) -> Vec<usize> {
    let mut q = std::collections::VecDeque::new();
    let mut dist = vec![usize::MAX; graph.len()];

    dist[start] = 0;
    q.push_back(start);

    while let Some(u) = q.pop_front() {
        for &v in &graph[u] {
            if dist[v] == usize::MAX {
                dist[v] = dist[u] + 1;
                q.push_back(v);
            }
        }
    }

    dist
}
