mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_array_sum(nums: &[i32], k: i32) -> i64 {
    let mut dp = vec![i64::MAX; 1 + k as usize];
    dp[0] = 0;
    let mut prefix = 0;
    for &num in nums.iter() {
        prefix += i64::from(num);
        let rem = (prefix % i64::from(k)) as usize;
        dp[rem] = dp[rem].min(prefix);
        prefix = dp[rem];
    }
    prefix
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
        assert_eq!(min_array_sum(&[1, 1, 1], 2), 1);
        assert_eq!(min_array_sum(&[3, 1, 4, 1, 5], 3), 5);
    }

    #[test]
    fn test() {}
}
