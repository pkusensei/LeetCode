mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let [n, delay, forget] = [n, delay, forget].map(|v| v as usize);
    let mut dp = vec![0_i64; 1 + n];
    dp[1] = 1;
    let mut curr = 0;
    for i in 2..=n {
        curr += dp[i.saturating_sub(delay)] - dp[i.saturating_sub(forget)];
        curr = curr.rem_euclid(MOD);
        dp[i] = curr;
    }
    dp[n - forget + 1..]
        .iter()
        .fold(0, |acc, v| (acc + v) % MOD) as i32
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
        assert_eq!(people_aware_of_secret(6, 2, 4), 5);
        assert_eq!(people_aware_of_secret(4, 1, 3), 6);
    }

    #[test]
    fn test() {}
}
