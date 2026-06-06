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
use itertools::Itertools;

pub fn max_score(nums: &[i32], max_val: i32) -> i32 {
    let mut max = max_val;
    let mut freq_num = HashMap::new();
    for &num in nums.iter() {
        max = max.max(num);
        *freq_num.entry(num).or_insert(0) += 1;
    }
    let mut freq_div = vec![0; 1 + max as usize];
    for div in 1..=max {
        for m in (div..=max).step_by(div as usize) {
            freq_div[div as usize] += freq_num.get(&m).unwrap_or(&0);
        }
    }
    let mut small_prime = (0..=max).collect_vec();
    for p in 2..=max.isqrt() {
        if small_prime[p as usize] == p {
            for val in (p * p..=max).step_by(p as usize) {
                if small_prime[val as usize] == val {
                    small_prime[val as usize] = p;
                }
            }
        }
    }
    let mut res = i32::MIN >> 1;
    let mut factors = [0; 10];
    for val in 1..=max {
        let cost;
        if val == 1 {
            cost = if freq_num.contains_key(&1) { 0 } else { 1 };
        } else {
            let mut curr = val;
            let mut i = 0;
            while curr > 1 {
                let p = small_prime[curr as usize];
                while curr % p == 0 {
                    curr /= p;
                }
                factors[i] = p;
                i += 1;
            }
            let mut bad = 0;
            for mask in 1_i32..(1 << i) {
                let mut div = 1;
                for bit in 0..i {
                    if (mask & (1 << bit)) > 0 {
                        div *= factors[bit];
                    }
                }
                if mask.count_ones() & 1 == 1 {
                    bad += freq_div[div as usize]
                } else {
                    bad -= freq_div[div as usize]
                }
            }
            cost = if freq_num.contains_key(&val) {
                bad - 1
            } else if val <= max_val {
                bad.max(1)
            } else {
                continue;
            };
        }
        res = res.max(val - cost);
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
        assert_eq!(max_score(&[3, 4, 6], 5), 4);
    }

    #[test]
    fn test() {}
}
