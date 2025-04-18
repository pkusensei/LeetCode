mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn leftmost_building_queries(heights: &[i32], queries: &[[i32; 2]]) -> Vec<i32> {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let mut res = vec![-1; queries.len()];
    let mut pending = vec![vec![]; heights.len()];
    for (idx, q) in queries.iter().enumerate() {
        let x = q[0].min(q[1]) as usize;
        let y = q[0].max(q[1]) as usize;
        if x == y || heights[x] < heights[y] {
            res[idx] = y as i32;
        } else {
            pending[y].push((idx, heights[x])); // h[x]>=h[y]
        }
    }
    let mut heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
    for (hi, &num) in heights.iter().enumerate() {
        while heap.peek().is_some_and(|v| v.0.0 < num) {
            let qi = heap.pop().unwrap().0.1;
            res[qi] = hi as i32;
        }
        for &(qi, q) in &pending[hi] {
            heap.push(Reverse((q, qi)));
        }
    }
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
            leftmost_building_queries(
                &[6, 4, 8, 5, 2, 7],
                &[[0, 1], [0, 3], [2, 4], [3, 4], [2, 2]]
            ),
            [2, 5, -1, 5, 2]
        );
        assert_eq!(
            leftmost_building_queries(
                &[5, 3, 8, 2, 6, 1, 4, 6],
                &[[0, 7], [3, 5], [5, 2], [3, 0], [1, 6]]
            ),
            [7, 6, -1, 4, 6]
        );
    }

    #[test]
    fn test() {}
}
