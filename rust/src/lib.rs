mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

struct NeighborSum {
    grid: Vec<Vec<i32>>,
    map: HashMap<i32, [i32; 2]>,
    rows: i32,
    cols: i32,
}

impl NeighborSum {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        let mut map = HashMap::new();
        let [mut rows, mut cols] = [0, 0];
        for (r, row) in grid.iter().enumerate() {
            for (c, &v) in row.iter().enumerate() {
                map.insert(v, [r as i32, c as i32]);
                rows = rows.max(1 + r as i32);
                cols = cols.max(1 + c as i32);
            }
        }
        Self {
            grid,
            map,
            rows,
            cols,
        }
    }

    fn adjacent_sum(&self, value: i32) -> i32 {
        let [r, c] = self.map[&value];
        let mut res = 0;
        for [dr, dc] in [[-1, 0], [1, 0], [0, -1], [0, 1]] {
            let rr = r + dr;
            let cc = c + dc;
            if (0..self.rows).contains(&rr) && (0..self.cols).contains(&cc) {
                res += self.grid[rr as usize][cc as usize];
            }
        }
        res
    }

    fn diagonal_sum(&self, value: i32) -> i32 {
        let [r, c] = self.map[&value];
        let mut res = 0;
        for [dr, dc] in [[-1, -1], [1, 1], [1, -1], [-1, 1]] {
            let rr = r + dr;
            let cc = c + dc;
            if (0..self.rows).contains(&rr) && (0..self.cols).contains(&cc) {
                res += self.grid[rr as usize][cc as usize];
            }
        }
        res
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
    fn basics() {}

    #[test]
    fn test() {}
}
