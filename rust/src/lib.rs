mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn min_set_bits(nums: Vec<i32>, k: i32) -> i32 {
    const B: usize = 17;
    let k = k as usize;
    let mut freq = [0; 17];
    let mut xs = HashSet::new();
    for (idx, &num) in nums.iter().enumerate() {
        for bit in 0..B {
            if num & (1 << bit) > 0 {
                freq[bit] += 1;
            }
        }
        if idx >= k {
            for bit in 0..B {
                if nums[idx - k] & (1 << bit) > 0 {
                    freq[bit] -= 1;
                }
            }
        }
        if idx >= k - 1 {
            let mut bit_and = 0;
            for (bit, &f) in freq.iter().enumerate() {
                if f == k {
                    bit_and |= 1 << bit;
                }
            }
            if bit_and == 0 {
                return -1;
            }
            xs.insert(bit_and);
        }
    }
    let mut res = B as i32;
    for y in 1_i32..(1 << B) {
        if y.count_ones() as i32 >= res {
            continue;
        }
        if xs.iter().all(|&x| x & y > 0) {
            res = y.count_ones() as i32;
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
