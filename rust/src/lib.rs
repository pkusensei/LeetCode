mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(nums1: &[i32], nums2: &[i32], k: i32) -> i64 {
    use itertools::Itertools;
    use std::{cmp::Reverse, collections::BinaryHeap};
    // sort desc on nums2 vals
    // so that when iterating, keep min(nums2)
    let pairs = nums2
        .iter()
        .zip(nums1.iter())
        .map(|(&a, &b)| [a, b].map(i64::from))
        .sorted_unstable_by(|a, b| b[0].cmp(&a[0]))
        .collect_vec();
    let mut sum = 0;
    let mut min = i64::MAX;
    let mut res = 0;
    // min heap to keep biggest nums1 in sum
    let mut heap = BinaryHeap::new();
    for p in pairs {
        // While in loop, ensure current p is in calculation
        sum += p[1];
        min = min.min(p[0]);
        heap.push(Reverse(p[1]));
        if heap.len() == k as usize {
            res = res.max(sum * min);
            let num = heap.pop().unwrap().0;
            sum -= num;
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
        assert_eq!(max_score(&[1, 3, 3, 2], &[2, 1, 3, 4], 3), 12);
        assert_eq!(max_score(&[4, 2, 3, 1, 1], &[7, 5, 10, 9, 6], 1), 30);
    }

    #[test]
    fn test() {}
}
