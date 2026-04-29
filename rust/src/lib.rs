mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(grid: Vec<Vec<i32>>) -> i64 {
    let n = grid.len();
    let mut memo = vec![vec![vec![-1; 1 + n]; 1 + n]; 1 + n];
    dfs(&grid, 0, 0, 0, &mut memo)
}

fn dfs(
    grid: &[Vec<i32>],
    idx: usize,
    height: usize,
    prev_height: usize,
    memo: &mut [Vec<Vec<i64>>],
) -> i64 {
    let n = grid.len();
    if idx >= n {
        return 0;
    }
    if memo[idx][height][prev_height] > -1 {
        return memo[idx][height][prev_height];
    }
    let mut from_p = (0..prev_height)
        .map(|i| i64::from(grid[i][idx]))
        .sum::<i64>();
    // Option 1: Nothing on this col is blocked
    // We take everything that prev col allows
    let mut res = from_p + dfs(grid, 1 + idx, 0, height, memo);
    // Option 2: Take one cell at a time and block it
    if 1 + idx < n {
        let mut sum = 0;
        for h in height..n {
            sum += i64::from(grid[h][idx]);
            // This `h` is blocked, prev_height=0 to next col
            res = res.max(sum + dfs(grid, 1 + idx, 1 + h, 0, memo));
        }
    }
    // Option 3: Block cell one by one
    for h in 0..n {
        if h < prev_height {
            // Remove contribution allowed by prev col
            from_p -= i64::from(grid[h][idx]);
        }
        res = res.max(from_p + dfs(grid, 1 + idx, 0, 1 + h, memo));
    }
    memo[idx][height][prev_height] = res;
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
