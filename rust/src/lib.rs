mod dsu;
mod helper;
mod trie;

use std::collections::VecDeque;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
    let n = grid.len();
    let mut queue = VecDeque::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &v) in row.iter().enumerate() {
            if v == 1 {
                queue.push_back([r, c]);
            }
        }
    }
    let mut dists = vec![vec![i32::MAX; n]; n];
    let mut dist = 0;
    while !queue.is_empty() {
        let len = queue.len();
        for _ in 0..len {
            let [r, c] = queue.pop_front().unwrap();
            dists[r][c] = dists[r][c].min(dist);
            for [nr, nc] in neighbors([r, c]) {
                if grid
                    .get(nr)
                    .is_some_and(|row| row.get(nc).is_some_and(|&v| v == 0))
                    && dists[nr][nc] > 1 + dist
                {
                    dists[nr][nc] = 1 + dist;
                    queue.push_back([nr, nc]);
                }
            }
        }
        dist += 1;
    }
    let mut left = 0;
    let mut right = dist;
    while left < right {
        let mid = left + (right + 1 - left) / 2;
        if check(&grid, &dists, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left
}

fn check(grid: &[Vec<i32>], dists: &[Vec<i32>], mid: i32) -> bool {
    let n = grid.len();
    if dists[0][0] < mid || dists[n - 1][n - 1] < mid {
        return false;
    }
    let mut queue = VecDeque::from([[0, 0]]);
    let mut seen = vec![vec![false; n]; n];
    seen[0][0] = true;
    while let Some([r, c]) = queue.pop_front() {
        if r == n - 1 && c == n - 1 {
            return true;
        }
        for [nr, nc] in neighbors([r, c]) {
            if dists
                .get(nr)
                .is_some_and(|row| row.get(nc).is_some_and(|&v| v >= mid))
                && !seen[nr][nc]
            {
                seen[nr][nc] = true;
                queue.push_back([nr, nc]);
            }
        }
    }
    false
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
            maximum_safeness_factor(vec![vec![1, 0, 0], vec![0, 0, 0], vec![0, 0, 1]]),
            0
        );
        assert_eq!(
            maximum_safeness_factor(vec![vec![0, 0, 1], vec![0, 0, 0], vec![0, 0, 0]]),
            2
        );
        assert_eq!(
            maximum_safeness_factor(vec![
                vec![0, 0, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 0, 0, 0],
                vec![1, 0, 0, 0]
            ]),
            2
        );
    }

    #[test]
    fn test() {
        assert_eq!(maximum_safeness_factor(vec![vec![1]]), 0);
    }
}
