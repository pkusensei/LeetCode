mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn maximum_product(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<_> = nums.into_iter().map(|v| Reverse(i64::from(v))).collect();
    for _ in 0..k {
        let Reverse(v) = heap.pop().unwrap();
        heap.push(Reverse(1 + v));
    }
    heap.into_iter()
        .fold(1, |acc, v| (acc * v.0) % 1_000_000_007) as i32
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
    fn basics() {}

    #[test]
    fn test() {}
}
