mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_coins(prices: &[i32]) -> i32 {
    let n = prices.len();
    let mut dp = vec![i32::MAX; 1 + n];
    dp[n] = 0;
    for left in (0..n).rev() {
        for right in 1 + left..=(2 + 2 * left).min(n) {
            dp[left] = dp[left].min(prices[left] + dp[right]);
        }
    }
    dp[0]
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
        assert_eq!(minimum_coins(&[3, 1, 2]), 4);
        assert_eq!(minimum_coins(&[1, 10, 1, 1]), 2);
        assert_eq!(minimum_coins(&[26, 18, 6, 12, 49, 7, 45, 45]), 39);
    }

    #[test]
    fn test() {}
}
