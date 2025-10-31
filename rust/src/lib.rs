mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut memo = vec![vec![vec![M; n]; n]; n];
    dfs(&grid, [0; 4], &mut memo).max(0)
}

const M: i32 = -2501;

fn dfs(grid: &[Vec<i32>], pos: [usize; 4], memo: &mut [Vec<Vec<i32>>]) -> i32 {
    let n = grid.len();
    let [r1, c1, r2, c2] = pos;
    if !pos.iter().all(|c| (0..n).contains(c)) || grid[r1][c1] == -1 || grid[r2][c2] == -1 {
        return M;
    }
    if r1 == n - 1 && c1 == n - 1 {
        return grid[c1][c1];
    }
    if memo[r1][c1][r2] > M {
        return memo[r1][c1][r2];
    }
    let mut res = grid[r1][c1];
    if r1 != r2 && c1 != c2 {
        res += grid[r2][c2];
    }
    res += [
        dfs(grid, [1 + r1, c1, 1 + r2, c2], memo),
        dfs(grid, [r1, 1 + c1, r2, 1 + c2], memo),
        dfs(grid, [1 + r1, c1, r2, 1 + c2], memo),
        dfs(grid, [r1, 1 + c1, 1 + r2, c2], memo),
    ]
    .into_iter()
    .max()
    .unwrap_or(0);
    memo[r1][c1][r2] = res;
    res
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
    fn basics() {}

    #[test]
    fn test() {}
}
