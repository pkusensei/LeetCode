mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    for val in 0..k {
        let mut dp = vec![0; k as usize];
        for &num in &nums {
            let prev = (k + val - num % k) % k;
            dp[(num % k) as usize] = 1 + dp[prev as usize];
            res = res.max(dp[(num % k) as usize]);
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
        assert_eq!(maximum_length(vec![1, 2, 3, 4, 5], 2), 5);
    }

    #[test]
    fn test() {}
}
