mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn leftmost_building_queries(heights: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let mut res = vec![-1; queries.len()];
    let mut pending = vec![vec![]; heights.len()];
    for (idx, q) in queries.iter().enumerate() {
        let x = q[0].min(q[1]) as usize;
        let y = q[0].max(q[1]) as usize;
        if x == y || heights[x] < heights[y] {
            res[idx] = y as i32;
        } else {
            // heights[x] >= heights[y]
            pending[y].push((idx, heights[x]));
        }
    }
    let mut stack = vec![];
    for (hi, &num) in heights.iter().enumerate().rev() {
        for &(pi, height) in pending[hi].iter() {
            let i = stack.partition_point(|&(_, v)| v > height);
            if let Some(&(_i, _h)) = i.checked_sub(1).and_then(|i| stack.get(i)) {
                res[pi] = _i as i32;
            }
        }
        // Scan from right to left
        // Maintain mono-increasing stack viewed from right
        // When doing stack.partition_point(|v|v>height),
        // the search result points to [i] <= height
        // Thus reading [i-1]
        while stack.last().is_some_and(|&(_, v)| v <= num) {
            stack.pop();
        }
        stack.push((hi, num));
    }
    res
}

fn with_pq(heights: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    let mut res = vec![-1; queries.len()];
    let mut pending = vec![vec![]; heights.len()];
    for (idx, q) in queries.iter().enumerate() {
        let x = q[0].min(q[1]) as usize;
        let y = q[0].max(q[1]) as usize;
        if x == y || heights[x] < heights[y] {
            res[idx] = y as i32;
        } else {
            // heights[x] >= heights[y]
            pending[y].push((idx, heights[x]));
        }
    }
    // min heap
    let mut heap = BinaryHeap::new();
    for (hi, &num) in heights.iter().enumerate() {
        while heap.peek().is_some_and(|&(Reverse(v), _)| v < num) {
            let Some((_, i)) = heap.pop() else {
                break;
            };
            res[i] = hi as i32;
        }
        for &(pi, height) in pending[hi].iter() {
            heap.push((Reverse(height), pi));
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            with_pq(
                &[6, 4, 8, 5, 2, 7],
                &[[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]]
            ),
            [2, 5, -1, 5, 2]
        );
        assert_eq!(
            with_pq(
                &[5, 3, 8, 2, 6, 1, 4, 6],
                &[[0, 7], [3, 5], [5, 2], [3, 0], [1, 6]]
            ),
            [7, 6, -1, 4, 6]
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

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
