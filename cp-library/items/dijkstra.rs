fn dijkstra(start: usize, graph: &[Vec<(usize, usize)>]) -> Vec<usize> {
    let mut dist = vec![usize::MAX; graph.len()];
    let mut heap = std::collections::BinaryHeap::new();

    dist[start] = 0;
    heap.push(std::cmp::Reverse((0, start)));

    while let Some(std::cmp::Reverse((d, u))) = heap.pop() {
        if d != dist[u] {
            continue;
        }

        for &(v, cost) in &graph[u] {
            let nd = d + cost;
            if chmin!(dist[v], nd) {
                heap.push(std::cmp::Reverse((nd, v)));
            }
        }
    }

    dist
}
