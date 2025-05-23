mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn find_x_sum(nums: &[i32], k: i32, x: i32) -> Vec<i32> {
    let (n, k, x) = (nums.len(), k as usize, x as usize);
    (0..=n - k).map(|i| x_sum(&nums[i..i + k], x)).collect()
}

fn x_sum(nums: &[i32], x: usize) -> i32 {
    let mut heap: BinaryHeap<_> = nums
        .iter()
        .fold(HashMap::new(), |mut acc, &v| {
            *acc.entry(v).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .map(|(num, count)| Reverse((count, num)))
        .collect();
    while heap.len() > x {
        heap.pop();
    }
    heap.iter().map(|Reverse((num, count))| num * count).sum()
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
        assert_eq!(find_x_sum(&[1, 1, 2, 2, 3, 4, 2, 3], 6, 2), [6, 10, 12]);
    }

    #[test]
    fn test() {}
}
