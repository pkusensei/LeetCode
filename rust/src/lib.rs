mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_paths(grid: &[&[i32]], k: i32) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    // let mut memo = vec![vec![vec![-1; k as usize]; cols]; rows];
    // dfs(grid, k, 0, 0, 0, &mut memo)
    let mut dp = vec![vec![vec![0; k as usize]; cols]; rows];
    dp[0][0][(grid[0][0] % k) as usize] = 1;
    for r in 0..rows {
        for c in 0..cols {
            for rem in 0..k {
                if 1 + r < rows {
                    let val = ((grid[1 + r][c] + rem) % k) as usize;
                    dp[1 + r][c][val] += dp[r][c][rem as usize];
                    dp[1 + r][c][val] %= 1_000_000_007;
                }
                if 1 + c < cols {
                    let val = ((grid[r][1 + c] + rem) % k) as usize;
                    dp[r][1 + c][val] += dp[r][c][rem as usize];
                    dp[r][1 + c][val] %= 1_000_000_007;
                }
            }
        }
    }
    dp[rows - 1][cols - 1][0]
}

fn dfs(grid: &[&[i32]], k: i32, r: usize, c: usize, curr: i32, memo: &mut [Vec<Vec<i32>>]) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    let curr = (curr + grid[r][c]) % k;
    if r == rows - 1 && c == cols - 1 {
        return i32::from(curr == 0);
    }
    if memo[r][c][curr as usize] > -1 {
        return memo[r][c][curr as usize];
    }
    let mut res = 0;
    if r + 1 < rows {
        res += dfs(grid, k, 1 + r, c, curr, memo);
    }
    if c + 1 < cols {
        res += dfs(grid, k, r, 1 + c, curr, memo);
    }
    res %= 1_000_000_007;
    memo[r][c][curr as usize] = res;
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
        assert_eq!(number_of_paths(&[&[5, 2, 4], &[3, 0, 5], &[0, 7, 2]], 3), 2);
        assert_eq!(number_of_paths(&[&[0, 0]], 5), 1);
        assert_eq!(
            number_of_paths(&[&[7, 3, 4, 9], &[2, 3, 6, 2], &[2, 3, 7, 0]], 1),
            10
        );
    }

    #[test]
    fn test() {}
}
