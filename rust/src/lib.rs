mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn split_array(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    if n <= 2 {
        return nums.iter().map(|&v| i64::from(v)).sum::<i64>().abs();
    }
    let mut sieve = vec![true; n];
    sieve[..2].fill(false);
    for p in 2..=n.isqrt() {
        if sieve[p] {
            for val in (p * p..n).step_by(p) {
                sieve[val] = false;
            }
        }
    }
    let mut res = 0_i64;
    for (i, &num) in nums.iter().enumerate() {
        if sieve[i] {
            res += i64::from(num)
        } else {
            res -= i64::from(num)
        }
    }
    res.abs()
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
