mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_paths_with_xor_value(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let k = k as usize;
    dfs(&grid, k, 0, 0, 0, &mut vec![vec![vec![-1; 17]; cols]; rows])
}

fn dfs(
    grid: &[Vec<i32>],
    k: usize,
    r: usize,
    c: usize,
    val: usize,
    memo: &mut [Vec<Vec<i32>>],
) -> i32 {
    let [rows, cols] = get_dimensions(grid);
    if r >= rows || c >= cols {
        return 0;
    }
    if memo[r][c][val] > -1 {
        return memo[r][c][val];
    }
    let curr = val ^ grid[r][c] as usize;
    if r == rows - 1 && c == cols - 1 {
        return i32::from(k == curr);
    }
    let res =
        (dfs(grid, k, 1 + r, c, curr, memo) + dfs(grid, k, r, 1 + c, curr, memo)) % 1_000_000_007;
    memo[r][c][val] = res;
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
            count_paths_with_xor_value(vec![vec![2, 1, 5], vec![7, 10, 0], vec![12, 6, 4]], 11),
            3
        );
    }

    #[test]
    fn test() {}
}
