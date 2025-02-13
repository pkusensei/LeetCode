mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<_> = nums.iter().map(|&v| Reverse(v as i64)).collect();
    let mut res = 0;
    while heap.len() > 1 && heap.peek().is_some_and(|&Reverse(v)| v < k as i64) {
        let x = heap.pop().unwrap().0;
        let y = heap.pop().unwrap().0;
        heap.push(Reverse(2 * x + y));
        res += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
