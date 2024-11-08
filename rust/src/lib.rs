mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn knight_dialer(n: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut prev = [1; 10];
    for _ in 1..n {
        let mut curr = [0; 10];
        curr[0] = (prev[4] + prev[6]) % MOD;
        curr[1] = (prev[6] + prev[8]) % MOD;
        curr[2] = (prev[7] + prev[9]) % MOD;
        curr[3] = (prev[4] + prev[8]) % MOD;
        curr[4] = (prev[0] + prev[3] + prev[9]) % MOD;
        // curr[5]=0;
        curr[6] = (prev[0] + prev[1] + prev[7]) % MOD;
        curr[7] = (prev[2] + prev[6]) % MOD;
        curr[8] = (prev[1] + prev[3]) % MOD;
        curr[9] = (prev[2] + prev[4]) % MOD;
        prev = curr;
    }
    prev.into_iter().fold(0, |acc, num| (acc + num) % MOD) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(knight_dialer(1), 10);
        debug_assert_eq!(knight_dialer(2), 20);
        debug_assert_eq!(knight_dialer(3131), 136006598);
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
