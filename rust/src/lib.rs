mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[allow(unused_imports)]
use helper::*;

pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
    let [k, x] = [k, x].map(|v| v as usize);
    let mut freq = HashMap::new();
    let mut res = vec![];
    for (i, &num) in nums.iter().enumerate() {
        *freq.entry(num).or_insert(0) += 1;
        if i >= k {
            *freq.entry(nums[i - k]).or_insert(0) -= 1;
        }
        if i >= k - 1 {
            let mut heap: BinaryHeap<_> = freq.iter().map(|(&num, &f)| Reverse((f, num))).collect();
            while heap.len() > x {
                heap.pop();
            }
            res.push(heap.into_iter().map(|Reverse((v, f))| v * f).sum());
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
    fn basics() {}

    #[test]
    fn test() {}
}
