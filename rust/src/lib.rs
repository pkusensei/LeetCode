mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(grid: &[&[i32]], k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let k = k as usize;
    let mut queue = BinaryHeap::new();
    let mut max_costs = vec![vec![vec![[i32::MAX >> 1; 4]; 1 + k]; cols]; rows];
    queue.push((Reverse(grid[0][0]), 0, 0, k, 1));
    max_costs[0][0][k][1] = grid[0][0];
    queue.push((Reverse(grid[0][0]), 0, 0, k, 3));
    max_costs[0][0][k][3] = grid[0][0];
    while let Some((Reverse(cost), r, c, turn, dir)) = queue.pop() {
        if r == rows - 1 && c == cols - 1 {
            return cost;
        }
        if cost > max_costs[r][c][turn][dir] {
            continue;
        }
        for (&didx, [dr, dc]) in DIDX.iter().zip(DIRS) {
            if didx == dir || turn > 0 {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if let Some([nr, nc]) = check(rows, cols, nr, nc) {
                    let ncost = grid[nr][nc] + cost;
                    let nturn = turn - usize::from(didx != dir);
                    if max_costs[nr][nc][nturn][didx] > ncost {
                        max_costs[nr][nc][nturn][didx] = ncost;
                        queue.push((Reverse(ncost), nr, nc, nturn, didx));
                    }
                }
            }
        }
    }
    -1
}

const fn check(rows: usize, cols: usize, r: i32, c: i32) -> Option<[usize; 2]> {
    if 0 <= r && r < rows as i32 && 0 <= c && c < cols as i32 {
        Some([r as usize, c as usize])
    } else {
        None
    }
}

const DIDX: [usize; 4] = [0, 1, 2, 3];
const DIRS: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];

pub fn with_dp(grid: &[&[i32]], k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let k = k as usize;
    let mut memo = vec![vec![vec![[-1; 4]; 1 + k]; cols]; rows];
    let res = dfs(&grid, 0, 0, k, 1, &mut memo).min(dfs(&grid, 0, 0, k, 3, &mut memo));
    if res < i32::MAX >> 1 { res } else { -1 }
}

fn dfs(
    grid: &[&[i32]],
    r: usize,
    c: usize,
    k: usize,
    dir: usize,
    memo: &mut [Vec<Vec<[i32; 4]>>],
) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    if r == rows - 1 && c == cols - 1 {
        return grid[r][c];
    }
    if memo[r][c][k][dir] > -1 {
        return memo[r][c][k][dir];
    }
    let mut res = i32::MAX >> 1;
    for (d, [dr, dc]) in DIRS.iter().enumerate() {
        if d == dir || k > 0 {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;
            if let Some([nr, nc]) = check(rows, cols, nr, nc) {
                let nk = k - usize::from(d != dir);
                res = res.min(dfs(grid, nr, nc, nk, d, memo));
            }
        }
    }
    res += grid[r][c];
    memo[r][c][k][dir] = res;
    res
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
        assert_eq!(min_cost(&[&[2, 7, 3], &[1, 4, 5]], 1), 12);
    }

    #[test]
    fn test() {}
}
