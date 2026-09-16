mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_dp(n: i32, k: i32) -> i32 {
    let [n, k] = [n, k].map(|v| v as usize);
    let mut dp = vec![vec![0; 1 + k]; n];
    for i in 0..n {
        dp[i][0] = 1;
    }
    for k in 1..=k {
        let mut pref = 0;
        for i in 1..n {
            pref = (pref + dp[i - 1][k - 1]) % M;
            dp[i][k] = (dp[i - 1][k] + pref) % M;
        }
    }
    dp[n - 1][k] as i32
}

pub fn number_of_sets(n: i32, k: i32) -> i32 {
    let [n, k] = [n, k].map(i64::from);
    let mut nom = 1;
    let mut den = 1;
    for i in 1..=2 * k {
        nom = nom * (n + k - i) % M;
        den = den * i % M;
    }
    (nom * mod_pow(den, M - 2) % M) as i32
}

const M: i64 = 1_000_000_007;
const fn mod_pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(base * base % M, exp >> 1)
    } else {
        mod_pow(base * base % M, exp >> 1) * base % M
    }
}

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
    fn basics() {}

    #[test]
    fn test() {}
}
