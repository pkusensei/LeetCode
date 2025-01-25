mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let [rows, cols] = get_dimensions(&heights);
    let mut dists = vec![vec![i32::MAX; cols]; rows];
    dists[0][0] = 0;
    let mut heap = BinaryHeap::from([(Reverse(0), [0, 0])]);
    while let Some((Reverse(cost), [r, c])) = heap.pop() {
        if r == rows - 1 && c == cols - 1 {
            return cost;
        }
        if cost > dists[r][c] {
            continue;
        }
        for [nr, nc] in neighbors([r, c]).filter(|&[nr, nc]| nr < rows && nc < cols) {
            let next_cost = (heights[r][c] - heights[nr][nc]).abs().max(cost);
            if next_cost < dists[nr][nc] {
                dists[nr][nc] = next_cost;
                heap.push((Reverse(next_cost), [nr, nc]));
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
