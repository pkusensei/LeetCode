mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_collected_fruits(fruits: &[&[i32]]) -> i32 {
    let n = fruits.len();
    let mut memo = vec![vec![-1; n]; n];
    let mut res = dfs1(&fruits, 0, n - 1, &mut memo) + dfs2(&fruits, n - 1, 0, &mut memo);
    res += (0..n).map(|i| fruits[i][i]).sum::<i32>();
    res
}

fn dfs1(grid: &[&[i32]], r: usize, c: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = grid.len();
    if r == n - 1 && c == n - 1 {
        return 0;
    }
    if r >= c {
        return 0;
    }
    if memo[r][c] > -1 {
        return memo[r][c];
    }
    let mut res = 0;
    for nc in [c.saturating_sub(1), c, 1 + c] {
        if nc < n {
            res = res.max(dfs1(grid, 1 + r, nc, memo));
        }
    }
    res += grid[r][c];
    memo[r][c] = res;
    res
}

fn dfs2(grid: &[&[i32]], r: usize, c: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = grid.len();
    if r == n - 1 && c == n - 1 {
        return 0;
    }
    if c >= r {
        return 0;
    }
    if memo[r][c] > -1 {
        return memo[r][c];
    }
    let mut res = 0;
    for nr in [r.saturating_sub(1), r, 1 + r] {
        if nr < n {
            res = res.max(dfs2(grid, nr, 1 + c, memo));
        }
    }
    res += grid[r][c];
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
        assert_eq!(
            max_collected_fruits(&[
                &[1, 2, 3, 4],
                &[5, 6, 8, 7],
                &[9, 10, 11, 12],
                &[13, 14, 15, 16]
            ]),
            100
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            max_collected_fruits(&[
                &[8, 5, 0, 17, 15],
                &[6, 0, 15, 6, 0],
                &[7, 19, 16, 8, 18],
                &[11, 3, 2, 12, 13],
                &[17, 15, 15, 4, 6]
            ]),
            145
        );
    }
}
