mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let sum1: i32 = nums1.iter().sum();
    let sum2: i32 = nums2.iter().sum();
    let (mut heap1, mut heap2) = match sum1.cmp(&sum2) {
        std::cmp::Ordering::Less => (
            BinaryHeap::from(nums2),
            BinaryHeap::from_iter(nums1.into_iter().map(Reverse)),
        ),
        std::cmp::Ordering::Equal => return 0,
        std::cmp::Ordering::Greater => (
            BinaryHeap::from(nums1),
            BinaryHeap::from_iter(nums2.into_iter().map(Reverse)),
        ),
    };
    let mut delta = (sum1 - sum2).abs();
    let mut res = 0;
    while delta > 0 {
        match (heap1.pop(), heap2.pop()) {
            (Some(a), Some(Reverse(b))) if a > 1 || b < 6 => {
                let trya = a - 1;
                let tryb = 6 - b;
                match trya.cmp(&tryb) {
                    std::cmp::Ordering::Less | std::cmp::Ordering::Equal => {
                        delta -= tryb;
                        heap1.push(a);
                    }
                    std::cmp::Ordering::Greater => {
                        delta -= trya;
                        heap2.push(Reverse(b));
                    }
                }
            }
            (Some(a), None) if a > 1 => delta -= a - 1,
            (None, Some(Reverse(b))) if b < 6 => delta -= 6 - b,
            _ => break,
        }
        res += 1
    }
    if delta <= 0 {
        res
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {

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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            min_operations(
                vec![5, 6, 4, 3, 1, 2],
                vec![6, 3, 3, 1, 4, 5, 3, 4, 1, 3, 4]
            ),
            4
        );
    }

    #[test]
    fn test() {}
}
