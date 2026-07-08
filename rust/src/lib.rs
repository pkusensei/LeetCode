mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn divisible_game(nums: Vec<i32>) -> i32 {
    const M: i64 = 1_000_000_007;
    let max = *nums.iter().max().unwrap();
    let mut res = i64::MIN;
    let mut min_k = 0;
    for &p in P.iter() {
        if p > max {
            break;
        }
        let curr = f(&nums, p);
        if curr > res {
            res = curr;
            min_k = p;
        }
    }
    if min_k == 0 {
        (-2_i64).rem_euclid(M) as i32
    } else {
        (res * i64::from(min_k)).rem_euclid(M) as i32
    }
}

fn f(nums: &[i32], k: i32) -> i64 {
    let arr = nums.iter().map(|&v| if v % k == 0 { v } else { -v });
    let mut curr = 0;
    let mut res = i64::MIN;
    for num in arr {
        let num = i64::from(num);
        curr = num.max(curr + num);
        res = res.max(curr);
    }
    res
}

const MAX: usize = 1_000_000;
static P: LazyLock<Vec<i32>> = LazyLock::new(|| {
    let mut sieve = vec![true; 1 + MAX];
    sieve[..2].fill(false);
    for p in 2..=MAX.isqrt() {
        if sieve[p] {
            for val in (p * p..=MAX).step_by(p) {
                sieve[val] = false
            }
        }
    }
    sieve
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v { Some(i as i32) } else { None })
        .collect()
});

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
