mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_tilings(n: i32) -> i32 {
    const M: i32 = 1_000_000_007;
    let n = n as usize;
    let mut dp = [0; 1001];
    dp[1] = 1;
    dp[2] = 2;
    dp[3] = 5;
    for i in 4..=n {
        dp[i] = (2 * dp[i - 1] % M + dp[i - 3]) % M
    }
    dp[n]
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
        assert_eq!(rotated_digits(10), 4);
    }

    #[test]
    fn test() {}
}
