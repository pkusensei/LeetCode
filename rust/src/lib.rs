mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
    // dfs(low, high, zero, one, 0, &mut vec![-1; 1 + high as usize])
    let [low, high, zero, one] = [low, high, zero, one].map(|v| v as usize);
    let mut dp = vec![0; 1 + high];
    for v in dp.iter_mut().skip(low) {
        *v = 1;
    }
    for i in (0..=high).rev() {
        if let Some(a) = i.checked_sub(zero) {
            dp[a] += dp[i];
            dp[a] %= MOD;
        }
        if let Some(b) = i.checked_sub(one) {
            dp[b] += dp[i];
            dp[b] %= MOD;
        }
    }
    dp[0]
}

const MOD: i32 = 1_000_000_007;

fn dfs(low: i32, high: i32, zero: i32, one: i32, curr: i32, memo: &mut [i32]) -> i32 {
    if curr > high {
        return 0;
    }
    if memo[curr as usize] > -1 {
        return memo[curr as usize];
    }
    let mut res = i32::from((low..=high).contains(&curr));
    res += dfs(low, high, zero, one, curr + zero, memo);
    res %= MOD;
    res += dfs(low, high, zero, one, curr + one, memo);
    res %= MOD;
    memo[curr as usize] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(count_good_strings(3, 3, 1, 1), 8);
        assert_eq!(count_good_strings(2, 3, 1, 2), 5);
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
