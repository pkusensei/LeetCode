mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let [rows, cols] = get_dimensions(&grid);
    let vals = grid
        .iter()
        .flatten()
        .copied()
        .sorted_unstable()
        .dedup()
        .collect_vec();
    // coordinate compression
    let val_id: HashMap<i32, usize> = vals.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut prev = vec![vec![i32::MAX / 2; cols]; rows];
    solve(&grid, &mut prev);
    let mut res = prev[rows - 1][cols - 1];
    for _ in 0..k {
        let vn = vals.len();
        let mut curr = vec![vec![i32::MAX / 2; cols]; rows];
        let mut best = vec![i32::MAX / 2; vn];
        for (r, row) in grid.iter().enumerate() {
            for (c, v) in row.iter().enumerate() {
                let i = val_id[v];
                // Min cost to reach each distinct value
                best[i] = best[i].min(prev[r][c]);
            }
        }
        // `vals` is sorted, vals[i]<=vals[1+i]
        // i.e the cost to reach [1+i] is good for all [..=i]
        // `best` is the suffix min cost to reach all vals
        for i in (0..vn - 1).rev() {
            best[i] = best[i].min(best[1 + i]);
        }
        for r in 0..rows {
            for c in 0..cols {
                // Propagate potential teleport steps
                curr[r][c] = prev[r][c].min(best[val_id[&grid[r][c]]]);
            }
        }
        solve(&grid, &mut curr);
        prev = curr;
        res = res.min(prev[rows - 1][cols - 1]);
    }
    res
}

fn solve(grid: &[Vec<i32>], dp: &mut [Vec<i32>]) {
    dp[0][0] = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if r > 0 {
                dp[r][c] = dp[r][c].min(v + dp[r - 1][c]);
            }
            if c > 0 {
                dp[r][c] = dp[r][c].min(v + dp[r][c - 1]);
            }
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
            min_cost(vec![vec![1, 3, 3], vec![2, 5, 4], vec![4, 3, 5]], 2),
            7
        );
        assert_eq!(min_cost(vec![vec![1, 2], vec![2, 3], vec![3, 4]], 1), 9);
    }

    #[test]
    fn test() {}
}
