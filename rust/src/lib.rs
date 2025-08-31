mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unique_paths(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut memo = vec![vec![[-1; 2]; cols]; rows];
    dfs(&grid, 0, 0, 0, &mut memo)
}

fn dfs(grid: &[Vec<i32>], r: usize, c: usize, dir: usize, memo: &mut [Vec<[i32; 2]>]) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    if r == rows - 1 && c == cols - 1 {
        return 1;
    }
    if r >= rows || c >= cols {
        return 0;
    }
    if memo[r][c][dir] > -1 {
        return memo[r][c][dir];
    }
    let mut res = if grid[r][c] == 0 {
        dfs(grid, 1 + r, c, 1, memo) + dfs(grid, r, 1 + c, 0, memo)
    } else if dir == 0 {
        dfs(grid, 1 + r, c, 1 - dir, memo)
    } else {
        dfs(grid, r, 1 + c, 1 - dir, memo)
    };
    res %= 1_000_000_007;
    memo[r][c][dir] = res;
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
    fn basics() {
        assert_eq!(
            unique_paths(vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 0, 0]]),
            5
        );
        assert_eq!(unique_paths(vec![vec![0, 0], vec![0, 0]]), 2);
        assert_eq!(unique_paths(vec![vec![0, 1, 1], vec![1, 1, 0]]), 1);
    }

    #[test]
    fn test() {}
}
