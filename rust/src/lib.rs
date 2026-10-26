mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn count_islands(mut grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut res = 0;
    let [rows, cols] = get_dimensions(&grid);
    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] > 0 {
                let v = bfs(&mut grid, r, c);
                res += i32::from(v % i64::from(k) == 0);
            }
        }
    }
    res
}

fn bfs(grid: &mut [Vec<i32>], r: usize, c: usize) -> i64 {
    let [rows, cols] = get_dimensions(grid);
    let mut res = 0;
    let mut queue = VecDeque::from([[r, c]]);
    res += i64::from(grid[r][c]);
    grid[r][c] = 0;
    while let Some([r, c]) = queue.pop_front() {
        for [nr, nc] in neighbors([r, c]) {
            if nr < rows && nc < cols && grid[nr][nc] > 0 {
                res += i64::from(grid[nr][nc]);
                grid[nr][nc] = 0;
                queue.push_back([nr, nc]);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
