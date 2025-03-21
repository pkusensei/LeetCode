mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let [low, high, zero, one] = [low, high, zero, one].map(|v| v as usize);
    let mut dp = vec![0; 1 + high];
    dp[0] = 1;
    for i in 0..=high {
        if i + zero <= high {
            dp[i + zero] = (dp[i + zero] + dp[i]) % MOD;
        }
        if i + one <= high {
            dp[i + one] = (dp[i + one] + dp[i]) % MOD;
        }
    }
    dp[low..=high].iter().fold(0, |acc, v| (acc + v) % MOD)
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
    fn basics() {}

    #[test]
    fn test() {}
}
