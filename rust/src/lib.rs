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

pub fn maximum_safeness_factor(grid: &[&[i32]]) -> i32 {
    let n = grid.len();
    let mut safe = vec![vec![i32::MAX; n]; n];
    let mut queue: VecDeque<_> = VecDeque::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == 1 {
                queue.push_back((r, c, 0));
                safe[r][c] = 0;
            }
        }
    }
    while let Some((r, c, v)) = queue.pop_front() {
        for [nr, nc] in neighbors([r, c]) {
            if nr < n && nc < n && safe[nr][nc] > 1 + v {
                safe[nr][nc] = 1 + v;
                queue.push_back((nr, nc, 1 + v));
            }
        }
    }
    let mut left = 0;
    let mut right = 2 * n as i32;
    while left < right {
        let mid = left + (1 + right - left) / 2;
        if check(&safe, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn check(safe: &[Vec<i32>], mid: i32) -> bool {
    let n = safe.len();
    if safe[0][0] < mid || safe[n - 1][n - 1] < mid {
        return false;
    }
    let mut queue = VecDeque::from([[0, 0]]);
    let mut seen = vec![vec![false; n]; n];
    seen[0][0] = true;
    while let Some([r, c]) = queue.pop_front() {
        if r == n - 1 && c == n - 1 {
            break;
        }
        for [nr, nc] in neighbors([r, c]) {
            if nr < n && nc < n && !seen[nr][nc] && safe[nr][nc] >= mid {
                seen[nr][nc] = true;
                queue.push_back([nr, nc]);
            }
        }
    }
    seen[n - 1][n - 1]
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
            maximum_safeness_factor(&[&[0, 0, 1], &[0, 0, 0], &[0, 0, 0]]),
            2
        );
        assert_eq!(
            maximum_safeness_factor(&[&[1, 0, 0], &[0, 0, 0], &[0, 0, 1]]),
            0
        );
    }

    #[test]
    fn test() {}
}
