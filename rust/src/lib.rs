mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_tilings(n: i32) -> i32 {
    let n = n as usize;
    let mut dp = vec![0; 1 + n];
    solve(n, &mut dp)
}

fn solve(n: usize, dp: &mut [i32]) -> i32 {
    const MOD: i32 = 1_000_000_007;
    if dp[n] > 0 {
        return dp[n];
    }
    let res = match n {
        1 => 1,
        2 => 2,
        3 => 5,
        _ => (2 * solve(n - 1, dp) % MOD + solve(n - 3, dp) % MOD) % MOD,
    };
    dp[n] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_tilings(3), 5);
        debug_assert_eq!(num_tilings(1), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(num_tilings(30), 312342182);
    }

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
}
