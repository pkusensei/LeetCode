mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_sliding_window(grid: &[&str], d: i32) -> i32 {
    let cols = grid[0].len();
    let d = d as usize;
    let mut dp = vec![];
    // rev?
    for row in grid.iter() {
        if dp.is_empty() {
            dp = slide(&vec![1; cols], row, 0);
        } else {
            dp = slide(&dp, row, (d.pow(2) - 1).isqrt());
        }
        dp = slide(&dp, row, d); // same row
    }
    dp.iter().fold(0, |acc, v| (acc + v) % M)
}

fn slide(prev: &Vec<i32>, row: &str, d: usize) -> Vec<i32> {
    let cols = prev.len();
    let mut curr = vec![0; cols];
    curr[0] = prev.iter().take(1 + d).fold(0, |acc, v| (acc + v) % M);
    for c in 1..cols {
        curr[c] = curr[c - 1];
        if c >= 1 + d {
            curr[c] = (curr[c] - prev[c - d - 1]).rem_euclid(M);
        }
        if c + d < cols {
            curr[c] = (curr[c] + prev[c + d]) % M;
        }
    }
    for (c, b) in row.bytes().enumerate() {
        if b == b'#' {
            curr[c] = 0;
        }
    }
    curr
}

pub fn number_of_routes(grid: &[&str], d: i32) -> i32 {
    use itertools::Itertools;
    let grid = grid
        .iter()
        .map(|s| s.bytes().map(|b| b == b'.').collect_vec())
        .collect_vec();
    let [rows, cols] = get_dimensions(&grid);
    let mut res = 0;
    for c in 0..cols {
        res += dfs(
            &grid,
            d.pow(2) as usize,
            0,
            c,
            0,
            &mut vec![vec![[-1; 2]; cols]; rows],
        );
        res %= M;
    }
    res
}

const M: i32 = 1_000_000_007;

fn dfs(
    grid: &[Vec<bool>],
    d: usize,
    row: usize,
    col: usize,
    stayed: usize,
    memo: &mut [Vec<[i32; 2]>],
) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    if !grid[row][col] {
        return 0;
    }
    if memo[row][col][stayed] > -1 {
        return memo[row][col][stayed];
    }
    memo[row][col][stayed] = if row == rows - 1 {
        if stayed == 1 {
            return 1;
        }
        let mut res = 1;
        for c in 0..cols {
            if (1..=d).contains(&c.abs_diff(col).pow(2)) {
                res += dfs(grid, d, row, c, 1, memo);
                res %= M;
            }
        }
        res
    } else {
        let mut res = 0;
        for c in 0..cols {
            if stayed == 0 && (1..=d).contains(&c.abs_diff(col).pow(2)) {
                res += dfs(grid, d, row, c, 1, memo);
                res %= M;
            }
            if c.abs_diff(col).pow(2) + 1 <= d {
                res += dfs(grid, d, 1 + row, c, 0, memo);
                res %= M;
            }
        }
        res
    };
    memo[row][col][stayed]
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
        assert_eq!(number_of_routes(&["..", "#."], 1), 2);
        assert_eq!(with_sliding_window(&["..", "#."], 1), 2);
    }

    #[test]
    fn test() {}
}
