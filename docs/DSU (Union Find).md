## Code 1

```rust
use ac_library::Dsu;

let mut dsu = Dsu::new(n);
for (a, b) in abs {
    // グループの併合
    dsu.merge(a, b);
}

// 同じグループか判定
dsu.same(1, 2);

// 要素の根を取得
dsu.leader(1);

// グループを取得
dsu.groups()
```


## Code 2

```rust
struct UnionFind {
    /// 各頂点の親頂点の番号. 自身が根のときは -1 とする.
    parents: Vec<i64>,
    /// 各頂点の属する根付き木の頂点数を表す.
    sizes: Vec<usize>,
}

#[allow(dead_code)]
impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parents: vec![-1; n],
            sizes: vec![1; n],
        }
    }

    /// 要素の根を取得
    fn root(&mut self, x: usize) -> usize {
        if self.parents[x] == -1 {
            // x が根の場合
            x
        } else {
            // 再帰的に実行する
            let root = self.root(self.parents[x] as usize);
            // 効率化のため, xの親を根にする (経路圧縮)
            self.parents[x] = root as i64;
            root
        }
    }

    /// x と y が同じグループか判定する
    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// x を含むグループと y を含むグループとを併合する
    fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y {
            return false;
        }

        // union by size (y 側のサイズが小さくなるようにする)
        if self.sizes[x] < self.sizes[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parents[y] = x as i64;
        self.sizes[x] += self.sizes[y];

        true
    }

    /// x を含むグループのサイズ
    fn size(&mut self, x: usize) -> usize {
        let i = self.root(x);
        self.sizes[i]
    }
}
```
