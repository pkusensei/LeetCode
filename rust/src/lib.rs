mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_servers(grid: &[&[i32]]) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut r_roots = vec![None; rows];
    let mut c_roots = vec![None; cols];
    let mut dsu = DSU::new(rows * cols);
    let mut id = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == 1 {
                if let Some(prev) = r_roots[r] {
                    dsu.union(prev, id);
                } else {
                    r_roots[r] = Some(id);
                }
                if let Some(prev) = c_roots[c] {
                    dsu.union(prev, id);
                } else {
                    c_roots[c] = Some(id);
                }
                id += 1;
            }
        }
    }
    (0..id)
        .filter(|&v| {
            let root = dsu.find(v);
            dsu.size[root] > 1
        })
        .count() as i32
}

struct DSU {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v])
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        if self.size[rx] < self.size[ry] {
            self.size[ry] += self.size[rx];
            self.parent[rx] = ry;
        } else {
            self.size[rx] += self.size[ry];
            self.parent[ry] = rx;
        }
    }
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(count_servers(&[&[1, 0], &[0, 1]]), 0);
    }

    #[test]
    fn test() {}
}
