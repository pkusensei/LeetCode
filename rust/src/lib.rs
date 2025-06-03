mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&coins);
    dfs(&coins, 0, 0, 2, &mut vec![vec![[None; 3]; cols]; rows])
}

fn dfs(grid: &[Vec<i32>], r: usize, c: usize, k: usize, memo: &mut [Vec<[Option<i32>; 3]>]) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    if r >= rows || c >= cols {
        return i32::MIN / 2;
    }
    if r == rows - 1 && c == cols - 1 {
        return if k > 0 { grid[r][c].max(0) } else { grid[r][c] };
    }
    if let Some(v) = memo[r][c][k] {
        return v;
    }
    let mut res = grid[r][c] + dfs(grid, 1 + r, c, k, memo).max(dfs(grid, r, 1 + c, k, memo));
    if k > 0 && grid[r][c] < 0 {
        res = res.max(dfs(grid, 1 + r, c, k - 1, memo).max(dfs(grid, r, 1 + c, k - 1, memo)));
    }
    memo[r][c][k] = Some(res);
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
            maximum_amount(vec![vec![0, 1, -1], vec![1, -2, 3], vec![2, -3, 4]]),
            8
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            maximum_amount(vec![
                vec![-7, 12, 12, 13],
                vec![-6, 19, 19, -6],
                vec![9, -2, -10, 16],
                vec![-4, 14, -10, -9]
            ]),
            60
        );
    }
}
