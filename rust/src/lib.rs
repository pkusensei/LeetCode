mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn furthest_building(heights: &[i32], mut bricks: i32, ladders: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    let mut prev_height = heights[0];
    for (i, &height) in heights.iter().enumerate().skip(1) {
        let diff = height - prev_height;
        if diff > 0 {
            heap.push(Reverse(diff)); // greedy; use ladder
            if heap.len() > ladders as usize {
                // out of ladders
                // find smallest gap and swap to bricks
                let Reverse(d) = heap.pop().unwrap();
                bricks -= d;
                if bricks < 0 {
                    return i as i32 - 1;
                }
            }
        }
        prev_height = height;
    }
    heights.len() as i32 - 1
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
    fn basics() {
        assert_eq!(furthest_building(&[4, 2, 7, 6, 9, 14, 12], 5, 1), 4);
        assert_eq!(
            furthest_building(&[4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
            7
        );
        assert_eq!(furthest_building(&[14, 3, 19, 3], 17, 0), 3);
    }

    #[test]
    fn test() {
        assert_eq!(furthest_building(&[1, 5, 1, 2, 3, 4, 10000], 4, 1), 5);
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
