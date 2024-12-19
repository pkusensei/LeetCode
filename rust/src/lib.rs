mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

const MOD: i32 = 1_000_000_007;

pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
    let alen = steps.min(arr_len);
    // let mut dp = vec![vec![-1; alen as usize]; 1 + steps as usize];
    // dfs(steps, alen, 0, &mut dp)
    let mut dp = vec![vec![0; alen as usize]; 1 + steps as usize];
    dp[0][0] = 1;
    for remain in 1..=steps as usize {
        for curr in (0..alen as usize).rev() {
            let mut temp = dp[remain - 1][curr];
            if curr > 0 {
                temp = (temp + dp[remain - 1][curr - 1]) % MOD;
            }
            if curr < alen as usize - 1 {
                temp = (temp + dp[remain - 1][curr + 1]) % MOD;
            }
            dp[remain][curr] = temp
        }
    }
    dp[steps as usize][0]
}

fn dfs(steps: i32, arr_len: i32, curr: i32, dp: &mut [Vec<i32>]) -> i32 {
    if steps == 0 {
        return i32::from(curr == 0);
    }
    if !(0..arr_len).contains(&curr) {
        return 0;
    }
    if dp[steps as usize][curr as usize] > -1 {
        return dp[steps as usize][curr as usize];
    }
    let mut res = 0;
    res = (res + dfs(steps - 1, arr_len, curr, dp)) % MOD;
    res = (res + dfs(steps - 1, arr_len, curr - 1, dp)) % MOD;
    res = (res + dfs(steps - 1, arr_len, curr + 1, dp)) % MOD;
    dp[steps as usize][curr as usize] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_ways(3, 2), 4);
        assert_eq!(num_ways(2, 4), 2);
        assert_eq!(num_ways(4, 2), 8);
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
