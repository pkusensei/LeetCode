mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn with_dp(grid: &[&[i32]], k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let vals = grid
        .iter()
        .flat_map(|r| r.iter().copied())
        .sorted_unstable()
        .dedup()
        .collect_vec();
    let map: HashMap<_, _> = vals.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    let mut dp = solve(&grid, vec![vec![i32::MAX >> 1; cols]; rows]);
    for _ in 0..k {
        let mut curr = vec![vec![i32::MAX >> 1; cols]; rows];
        let mut least_cost = vec![i32::MAX >> 1; vals.len()];
        for (r, row) in grid.iter().enumerate() {
            for (c, v) in row.iter().enumerate() {
                let i = map[v];
                // For each value, track least cost to jump to
                least_cost[i] = least_cost[i].min(dp[r][c]);
            }
        }
        // suffix min
        // For `v1<v2`, all jump to `v2` is valid for `v1`
        for i in (0..vals.len() - 1).rev() {
            least_cost[i] = least_cost[i].min(least_cost[1 + i]);
        }
        for (r, row) in curr.iter_mut().enumerate() {
            for (c, v) in row.iter_mut().enumerate() {
                *v = dp[r][c].min(least_cost[map[&grid[r][c]]]);
            }
        }
        dp = solve(grid, curr);
    }
    dp[rows - 1][cols - 1]
}

fn solve(grid: &[&[i32]], mut dp: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
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
    dp
}

// Suppose grid is one row
// [[1, 9, 8, 7, 12, 9, 999, 12, 14]]
//   0  1  2  3   4  5    6   7   8
// When we see [1]==9, we unlock jumps to all val<=9
// i.e 1->9->...->[5]->[6]->...
// But here's behemoth  ^
// Otherwise we save the jump and see [4]==12
// We unlock jump to [7]==12
// Now we can do 1->9->...->[4]->[7]->[8]
// The upper bound of a jump is limited by values we could reach.

pub fn min_cost(grid: &[&[i32]], k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let map = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter().enumerate().map(move |(c, &v)| (r, c, v)))
        .fold(BTreeMap::<_, Vec<_>>::new(), |mut acc, (r, c, v)| {
            acc.entry(v).or_default().push([r, c]);
            acc
        });
    let k = k as usize;
    let mut dists = vec![vec![vec![i32::MAX >> 1; 1 + k]; cols]; rows];
    dists[0][0][k] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), 0, 0, k)]);
    let mut max_expanded = vec![-1; k];
    while let Some((Reverse(cost), r, c, rem_k)) = heap.pop() {
        if cost > dists[r][c][rem_k] {
            continue;
        }
        if r == rows - 1 && c == cols - 1 {
            return cost;
        }
        for [nr, nc] in [[1 + r, c], [r, 1 + c]] {
            if let Some(v) = grid.get(nr).and_then(|row| row.get(nc)) {
                let ncost = cost + v;
                if ncost < dists[nr][nc][rem_k] {
                    dists[nr][nc][rem_k] = ncost;
                    heap.push((Reverse(ncost), nr, nc, rem_k));
                }
            }
        }
        // Tracks max grid value before reaching `nk` teleports
        // Suppose this cell has value `v1`, we could jump to all `v<=v1`
        // Later when seeing `v2<=v1`, no jump is useful (at same `nk`)
        if let Some(nk) = rem_k.checked_sub(1)
            && max_expanded[nk] < grid[r][c]
        {
            let start = 1 + max_expanded[nk];
            for (_, pos) in map.range(start..=grid[r][c]) {
                for &[nr, nc] in pos {
                    if cost < dists[nr][nc][nk] {
                        dists[nr][nc][nk] = cost;
                        heap.push((Reverse(cost), nr, nc, nk));
                    }
                }
            }
            max_expanded[nk] = grid[r][c];
        }
    }
    -1
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
        assert_eq!(min_cost(&[&[1, 3, 3], &[2, 5, 4], &[4, 3, 5]], 2), 7);
        assert_eq!(min_cost(&[&[1, 2], &[2, 3], &[3, 4]], 1), 9);
        assert_eq!(
            min_cost(
                &[
                    &[3, 0, 0],
                    &[3, 1, 2],
                    &[4, 4, 3],
                    &[5, 4, 6],
                    &[8, 6, 4],
                    &[9, 8, 6],
                    &[11, 8, 8],
                    &[11, 7, 12],
                    &[12, 10, 9]
                ],
                6
            ),
            23
        );

        assert_eq!(with_dp(&[&[1, 3, 3], &[2, 5, 4], &[4, 3, 5]], 2), 7);
        assert_eq!(with_dp(&[&[1, 2], &[2, 3], &[3, 4]], 1), 9);
        assert_eq!(
            with_dp(
                &[
                    &[3, 0, 0],
                    &[3, 1, 2],
                    &[4, 4, 3],
                    &[5, 4, 6],
                    &[8, 6, 4],
                    &[9, 8, 6],
                    &[11, 8, 8],
                    &[11, 7, 12],
                    &[12, 10, 9]
                ],
                6
            ),
            23
        );
    }

    #[test]
    fn test() {}
}
