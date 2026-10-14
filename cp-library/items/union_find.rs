struct UnionFind {
    parents: Vec<usize>,
    sizes: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: (0..n).collect(),
            sizes: vec![1; n],
        }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parents[x] != x {
            self.parents[x] = self.root(self.parents[x]);
        }
        self.parents[x]
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return false;
        }
        if self.sizes[x] < self.sizes[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parents[y] = x;
        self.sizes[x] += self.sizes[y];
        true
    }

    fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        self.sizes[root]
    }
}
