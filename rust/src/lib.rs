mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::{collections::HashMap, sync::LazyLock};

#[allow(unused_imports)]
use helper::*;

pub fn longest_subarray(nums: &[i32], k: i32) -> i32 {
    let k = k as usize;
    let mut map = HashMap::new();
    let mut left = 0;
    let mut res = 0;
    let mut seen = vec![];
    for (right, &num) in nums.iter().enumerate() {
        let num = num as usize;
        let mut curr = vec![];
        for p in 1..=num.isqrt() {
            if num % p == 0 {
                if P[p] {
                    curr.push(p);
                    *map.entry(p).or_insert(0) += 1;
                }
                if p * p != num && P[num / p] {
                    curr.push(num / p);
                    *map.entry(num / p).or_insert(0) += 1;
                }
            }
        }
        if curr.len() > k {
            map.clear();
            left = 1 + right;
            seen.push(vec![]);
            continue;
        }
        seen.push(curr);
        while map.len() > k && left <= right {
            for &p in &seen[left] {
                let v = map.entry(p).or_insert(0);
                *v -= 1;
                if *v == 0 {
                    map.remove(&p);
                }
            }
            left += 1;
        }
        res = res.max(1 + right - left);
    }
    res as i32
}

static P: LazyLock<Vec<bool>> = LazyLock::new(|| {
    const M: usize = 100_000;
    let mut sieve = vec![true; 1 + M];
    sieve[..2].fill(false);
    for p in 2..=M.isqrt() {
        if sieve[p] {
            for val in (p * p..=M).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    sieve
});

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
    fn basics() {
        assert_eq!(longest_subarray(&[7, 6, 10, 12, 11], 3), 3);
    }

    #[test]
    fn test() {}
}
