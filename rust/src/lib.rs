mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    dfs(&grid, [0, 0], [rows, cols], 2, &mut HashMap::new()).unwrap_or(0)
}

fn dfs(
    grid: &[Vec<i32>],
    mins: [usize; 2],
    maxs: [usize; 2],
    splits: u8,
    memo: &mut HashMap<([usize; 2], [usize; 2], usize), Option<i32>>,
) -> Option<i32> {
    if splits == 0 {
        return min_area(grid, mins, maxs, memo);
    }
    let [min_r, min_c] = mins;
    let [max_r, max_c] = maxs;
    let mut res = None;
    for r in 1 + min_r..max_r {
        res = update(
            res,
            min_area(grid, mins, [r, max_c], memo),
            dfs(grid, [r, min_c], maxs, splits - 1, memo),
        );
        res = update(
            res,
            dfs(grid, mins, [r, max_c], splits - 1, memo),
            min_area(grid, [r, min_c], maxs, memo),
        );
    }
    for c in 1 + min_c..max_c {
        res = update(
            res,
            min_area(grid, mins, [max_r, c], memo),
            dfs(grid, [min_r, c], maxs, splits - 1, memo),
        );
        res = update(
            res,
            dfs(grid, mins, [max_r, c], splits - 1, memo),
            min_area(grid, [min_r, c], maxs, memo),
        );
    }
    res
}

fn update(mut res: Option<i32>, a: Option<i32>, b: Option<i32>) -> Option<i32> {
    if let Some((a, b)) = a.zip(b) {
        if res.is_none_or(|v| v > a + b) {
            res = Some(a + b)
        }
    }
    res
}

fn min_area(
    grid: &[Vec<i32>],
    mins: [usize; 2],
    maxs: [usize; 2],
    memo: &mut HashMap<([usize; 2], [usize; 2], usize), Option<i32>>,
) -> Option<i32> {
    let key = (mins, maxs, 0);
    if let Some(&v) = memo.get(&key) {
        return v;
    }
    let [mut min_r, mut min_c] = maxs;
    let [mut max_r, mut max_c] = mins;
    for r in mins[0]..maxs[0] {
        for c in mins[1]..maxs[1] {
            if grid[r][c] == 1 {
                min_r = min_r.min(r);
                min_c = min_c.min(c);
                max_r = max_r.max(r);
                max_c = max_c.max(c);
            }
        }
    }
    let res = max_r
        .checked_sub(min_r)
        .zip(max_c.checked_sub(min_c))
        .map(|(dr, dc)| ((1 + dr) * (1 + dc)) as i32);
    memo.insert(key, res);
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
        assert_eq!(minimum_sum(vec![vec![1, 0, 1], vec![1, 1, 1]]), 5);
        assert_eq!(minimum_sum(vec![vec![1, 0, 1, 0], vec![0, 1, 0, 1]]), 5);
    }

    #[test]
    fn test() {}
}
