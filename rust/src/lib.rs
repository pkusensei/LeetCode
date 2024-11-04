mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

const MOD: i32 = 1_000_000_007;

pub fn num_perms_di_sequence(s: &str) -> i32 {
    let n = s.len();
    let mut dp = vec![vec![-1; 1 + n]; 1 + n];
    (0..=n)
        .map(|i| solve(s.as_bytes(), i, &mut dp))
        .fold(0, |acc, v| (acc + v) % MOD)

    // let mut dp = vec![vec![0; 1 + n]; 1 + n];
    // dp[0] = vec![1; 1 + n];
    // for (i1, b) in s.bytes().enumerate() {
    //     let mut curr = 0;
    //     if b == b'D' {
    //         for i2 in 0..n - i1 {
    //             curr = (curr + dp[i1][i2]) % MOD;
    //             dp[1 + i1][i2] = curr;
    //         }
    //     } else {
    //         for i2 in (0..n - i1).rev() {
    //             curr = (curr + dp[i1][1 + i2]) % MOD;
    //             dp[1 + i1][i2] = curr;
    //         }
    //     }
    // }
    // dp[n][0]
}

fn solve(s: &[u8], idx: usize, dp: &mut [Vec<i32>]) -> i32 {
    if s.is_empty() {
        return 1;
    }
    let n = s.len();
    if dp[n][idx] > -1 {
        return dp[n][idx];
    }
    let mut res = 0;
    match s[0] {
        b'I' => {
            for i in 1 + idx..=n {
                res = (res + solve(&s[1..], i - 1, dp)) % MOD;
            }
        }
        _ => {
            for i in (0..idx).rev() {
                res = (res + solve(&s[1..], i, dp)) % MOD;
            }
        }
    }
    dp[n][idx] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_perms_di_sequence("DID"), 5);
        debug_assert_eq!(num_perms_di_sequence("D"), 1);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
