mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
    let [rows, cols] = get_dimensions(&grid);
    let mut seen = vec![vec![false; cols]; rows];
    let mut res = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v > 0 && !seen[r][c] {
                res += bfs(&grid, k, r, c, &mut seen);
            }
        }
    }
    res
}

fn bfs(grid: &[Vec<i32>], k: i32, r: usize, c: usize, seen: &mut [Vec<bool>]) -> i32 {
    let mut queue = VecDeque::from([[r, c]]);
    let mut sum = grid[r][c];
    seen[r][c] = true;
    while let Some([r, c]) = queue.pop_front() {
        for [nr, nc] in neighbors([r, c]).filter(|&[nr, nc]| {
            grid.get(nr)
                .is_some_and(|row| row.get(nc).is_some_and(|&v| v > 0))
        }) {
            if !seen[nr][nc] {
                seen[nr][nc] = true;
                queue.push_back([nr, nc]);
                sum += grid[nr][nc];
            }
        }
    }
    i32::from(sum % k == 0)
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
    fn basics() {}

    #[test]
    fn test() {}
}
