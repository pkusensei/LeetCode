mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn min_side_jumps(obstacles: &[i32]) -> i32 {
    let n = obstacles.len();
    let mut costs = vec![vec![1_000_000; 1 + n]; 3];
    // (Reverse(cost), idx, lane)
    let mut heap = BinaryHeap::from([(Reverse(0), 0, 1)]);
    while let Some((Reverse(cost), idx, lane)) = heap.pop() {
        if idx == n - 1 {
            return cost;
        }
        if cost > costs[lane][idx] {
            continue;
        }
        if obstacles[1 + idx] - 1 != lane as i32 {
            heap.push((Reverse(cost), 1 + idx, lane));
            costs[lane][1 + idx] = cost;
        }
        for next in 0..3 {
            let nc = 1 + cost;
            if next != lane && obstacles[idx] - 1 != next as i32 && costs[next][idx] > nc {
                costs[next][idx] = nc;
                heap.push((Reverse(nc), idx, next));
            }
        }
    }
    -1
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
        assert_eq!(min_side_jumps(&[0, 1, 2, 3, 0]), 2);
        assert_eq!(min_side_jumps(&[0, 1, 1, 3, 3, 0]), 0);
        assert_eq!(min_side_jumps(&[0, 2, 1, 0, 3, 0]), 2);
    }

    #[test]
    fn test() {}
}
