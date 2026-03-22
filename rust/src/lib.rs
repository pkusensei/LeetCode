mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

fn build_subseqs(nums: &[i32]) -> Vec<i32> {
    fn f_acc(acc: i32, num: i32) -> i32 {
        todo!() // stub
    }

    let n = nums.len();
    let full_mask = 1 << n;
    let mut res = Vec::with_capacity(full_mask);
    res.push(0); // identity
    for mask in 1..full_mask {
        let lsb = mask & mask.wrapping_neg();
        let curr = f_acc(res[mask ^ lsb], nums[lsb.ilog2() as usize]);
        res.push(curr);
    }
    res
}

pub fn min_removals(nums: &[i32], target: i32) -> i32 {
    // (sub_or, min_len)
    let mut prev = HashMap::from([(0, 0)]);
    for &num in nums.iter() {
        let mut curr = prev.clone();
        for (v, len) in prev {
            let val = v ^ num;
            let temp = curr.entry(val).or_insert(1 + len);
            *temp = (*temp).min(1 + len);
        }
        prev = curr
    }
    let xor = nums.iter().fold(target, |acc, v| acc ^ v);
    *prev.get(&xor).unwrap_or(&-1)
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
        assert_eq!(min_removals(&[7], 7), 0);
    }

    #[test]
    fn test() {}
}
