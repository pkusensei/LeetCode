mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn latest_day_to_cross(row: i32, col: i32, cells: &[[i32; 2]]) -> i32 {
    let [rows, cols] = [row, col].map(|v| v as usize);
    let mut blocked: HashSet<_> = cells
        .iter()
        .map(|c| [c[0], c[1]].map(|v| v as usize - 1))
        .collect();
    let mut dsu = DSU::new(2 + rows * cols);
    for c in 0..cols {
        dsu.union(rows * cols, c); // union top row with virtual node rows*cols
        dsu.union(1 + rows * cols, (rows - 1) * cols + c); // bottom row to another
    }
    for (i, c) in cells.iter().enumerate().rev() {
        let [row, col] = [c[0], c[1]].map(|v| v as usize - 1);
        blocked.remove(&[row, col]);
        for [nr, nc] in neighbors([row, col]) {
            if nr < rows && nc < cols && !blocked.contains(&[nr, nc]) {
                dsu.union(row * cols + col, nr * cols + nc);
            }
        }
        if dsu.find(rows * cols) == dsu.find(1 + rows * cols) {
            return i as i32;
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
            self.parent[v] = self.find(self.parent[v])
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
            latest_day_to_cross(2, 2, &[[1, 1], [2, 1], [1, 2], [2, 2]]),
            2
        );
        assert_eq!(
            latest_day_to_cross(2, 2, &[[1, 1], [1, 2], [2, 1], [2, 2]]),
            1
        );
        assert_eq!(
            latest_day_to_cross(
                3,
                3,
                &[
                    [1, 2],
                    [2, 1],
                    [3, 3],
                    [2, 2],
                    [1, 1],
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [3, 1]
                ]
            ),
            3
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            latest_day_to_cross(
                3,
                4,
                &[
                    [3, 1],
                    [2, 3],
                    [1, 3],
                    [3, 2],
                    [2, 1],
                    [1, 4],
                    [1, 1],
                    [2, 4],
                    [3, 4],
                    [1, 2],
                    [2, 2],
                    [3, 3]
                ]
            ),
            5
        );
    }
}
