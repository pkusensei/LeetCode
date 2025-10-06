mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn swim_in_water(grid: &[&[i32]]) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = grid.len();
    let mut heap = BinaryHeap::from([(Reverse(grid[0][0]), 0, 0)]);
    let mut seen = vec![vec![1 + n.pow(2) as i32; n]; n];
    while let Some((Reverse(step), r, c)) = heap.pop() {
        if r == n - 1 && c == n - 1 {
            return step;
        }
        if step > seen[r][c] {
            continue;
        }
        for [nr, nc] in neighbors([r, c]) {
            if let Some(&v) = grid.get(nr).and_then(|row| row.get(nc)) {
                let next = v.max(step);
                if next < seen[nr][nc] {
                    seen[nr][nc] = next;
                    heap.push((Reverse(next), nr, nc));
                }
            }
        }
    }
    -1
}

pub fn with_dsu(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    let mut vals = Vec::with_capacity(n * n);
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            vals.push((v, r, c));
        }
    }
    vals.sort_unstable_by_key(|v| v.0);
    let mut pass = vec![false; n * n];
    let mut dsu = DSU::new(n * n);
    for (step, r, c) in vals {
        pass[r * n + c] = true;
        for [nr, nc] in neighbors([r, c]) {
            if nr < n && nc < n && pass[nr * n + nc] {
                dsu.union(r * n + c, nr * n + nc);
            }
        }
        if dsu.find(0) == dsu.find(n * n - 1) {
            return step;
        }
    }
    -1
}

struct DSU {
    parent: Vec<usize>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    fn find(&mut self, v: usize) -> usize {
        if self.parent[v] != v {
            self.parent[v] = self.find(self.parent[v]);
        }
        self.parent[v]
    }

    fn union(&mut self, x: usize, y: usize) {
        let [rx, ry] = [x, y].map(|v| self.find(v));
        if rx == ry {
            return;
        }
        match self.rank[rx].cmp(&self.rank[ry]) {
            std::cmp::Ordering::Less => self.parent[rx] = ry,
            std::cmp::Ordering::Equal => {
                self.parent[ry] = rx;
                self.rank[rx] += 1;
            }
            std::cmp::Ordering::Greater => self.parent[ry] = rx,
        }
    }
}

#[cfg(test)]
mod tests {

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
        assert_eq!(
            swim_in_water(&[
                &[0, 1, 2, 3, 4],
                &[24, 23, 22, 21, 5],
                &[12, 13, 14, 15, 16],
                &[11, 17, 18, 19, 20],
                &[10, 9, 8, 7, 6]
            ]),
            16
        );
        assert_eq!(
            with_dsu(&[
                &[0, 1, 2, 3, 4],
                &[24, 23, 22, 21, 5],
                &[12, 13, 14, 15, 16],
                &[11, 17, 18, 19, 20],
                &[10, 9, 8, 7, 6]
            ]),
            16
        );
    }

    #[test]
    fn test() {}
}
