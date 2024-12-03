mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

const MOD: i32 = 1_000_000_007;

pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
    let [n, k, t] = [n, k, target].map(|v| v as usize);
    // dfs(n, k, t, &mut vec![vec![-1; 1 + n]; 1 + t])
    let mut dp = vec![0; 1 + t];
    dp[0] = 1;
    for _ in 0..n {
        let mut next = vec![0; 1 + t];
        for num in 1..=t {
            for face in 1..=k {
                if num >= face {
                    next[num] = (next[num] + dp[num - face]) % MOD;
                }
            }
        }
        dp = next;
    }
    dp[t]
}

fn dfs(n: usize, k: usize, t: usize, dp: &mut [Vec<i32>]) -> i32 {
    if n == 1 && (1..=k).contains(&t) {
        return 1;
    }
    if n <= 1 {
        return 0;
    }
    if dp[t][n] > -1 {
        return dp[t][n];
    }
    let mut res = 0;
    for i in 1..=(k).min(t) {
        res = (res + dfs(n - 1, k, t - i, dp)) % MOD;
    }
    dp[t][n] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_rolls_to_target(1, 6, 3), 1);
        assert_eq!(num_rolls_to_target(2, 6, 7), 6);
        assert_eq!(num_rolls_to_target(30, 30, 500), 222616187);
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
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
