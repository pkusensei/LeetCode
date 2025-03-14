mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_paths(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut memo = vec![vec![-1; cols]; rows];
    let mut res = 0;
    for r in 0..rows {
        for c in 0..cols {
            res += dfs(&grid, r, c, &mut memo);
            res %= MOD;
        }
    }
    res
}

const MOD: i32 = 1_000_000_007;

fn dfs(grid: &[Vec<i32>], r: usize, c: usize, memo: &mut [Vec<i32>]) -> i32 {
    if memo[r][c] > -1 {
        return memo[r][c];
    }
    let mut res = 1;
    for [nr, nc] in neighbors([r, c]) {
        if grid
            .get(nr)
            .is_some_and(|row| row.get(nc).is_some_and(|&v| v > grid[r][c]))
        {
            res += dfs(grid, nr, nc, memo);
            res %= MOD;
        }
    }
    memo[r][c] = res;
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
        assert_eq!(count_paths(vec![vec![1, 1], vec![3, 4]]), 8);
        assert_eq!(count_paths(vec![vec![1], vec![2]]), 3);
    }

    #[test]
    fn test() {}
}
