mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let k = k as usize;
    dfs(
        &grid,
        k,
        0,
        0,
        0,
        &mut vec![vec![vec![None; 1 + k]; cols]; rows],
    )
    .unwrap_or(-1)
}

fn dfs(
    grid: &[Vec<i32>],
    k: usize,
    row: usize,
    col: usize,
    cost: usize,
    memo: &mut [Vec<Vec<Option<i32>>>],
) -> Option<i32> {
    let [rows, cols] = get_dimensions(grid);
    if row >= rows || col >= cols {
        return None;
    }
    if let Some(v) = memo[row][col][cost] {
        return if v < 0 { None } else { Some(v) };
    }
    let ncost = cost + usize::from(grid[row][col] > 0);
    if ncost > k {
        return None;
    }
    let curr = grid[row][col];
    if row == rows - 1 && col == cols - 1 {
        return Some(curr);
    }
    if let Some(res) = match (
        dfs(grid, k, 1 + row, col, ncost, memo),
        dfs(grid, k, row, 1 + col, ncost, memo),
    ) {
        (None, None) => None,
        (None, Some(v)) | (Some(v), None) => Some(curr + v),
        (Some(a), Some(b)) => Some(curr + a.max(b)),
    } {
        memo[row][col][cost] = Some(res);
        Some(res)
    } else {
        memo[row][col][cost] = Some(-1);
        None
    }
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
        assert_eq!(max_path_score(vec![vec![0, 1], vec![2, 0]], 1), 2);
        assert_eq!(max_path_score(vec![vec![0, 1], vec![1, 2]], 1), -1);
    }

    #[test]
    fn test() {}
}
