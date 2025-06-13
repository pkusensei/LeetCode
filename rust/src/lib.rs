mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_sum(nums1: &[i32], nums2: &[i32], k: i32) -> Vec<i64> {
    use itertools::{Itertools, izip};
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = nums1.len();
    let k = k as usize;
    let sorted = izip!(nums1.iter(), nums2.iter())
        .enumerate()
        .map(|(i, (&v1, &v2))| (v1, i, v2))
        .sorted_unstable()
        .collect_vec();
    let mut heap = BinaryHeap::<Reverse<i64>>::with_capacity(1 + k);
    let mut res = vec![0; n];
    let mut left = 0;
    let mut sum = 0;
    for &(v1, idx, _) in &sorted {
        while sorted[left].0 < v1 {
            let v2 = i64::from(sorted[left].2);
            heap.push(Reverse(v2));
            sum += v2;
            if heap.len() > k {
                sum -= heap.pop().unwrap().0;
            }
            left += 1;
        }
        res[idx] = sum;
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
            find_max_sum(&[4, 2, 1, 5, 3], &[10, 20, 30, 40, 50], 2),
            [80, 30, 0, 80, 50]
        );
        assert_eq!(find_max_sum(&[2, 2, 2, 2], &[3, 1, 2, 3], 1), [0, 0, 0, 0]);
    }

    #[test]
    fn test() {}
}
