mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_vowel_permutation(n: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    // a e i o u
    // 0 1 2 3 4
    let mut curr = [1; 5];
    for _ in 1..n {
        let mut next = [0; 5];
        // ae
        next[1] = (next[1] + curr[0]) % MOD;
        // ea
        next[0] = (next[0] + curr[1]) % MOD;
        // ei
        next[2] = (next[2] + curr[1]) % MOD;
        // ia, ie, io, iu
        for i in [0, 1, 3, 4] {
            next[i] = (next[i] + curr[2]) % MOD;
        }
        // oi
        next[2] = (next[2] + curr[3]) % MOD;
        // ou
        next[4] = (next[4] + curr[3]) % MOD;
        // ua
        next[0] = (next[0] + curr[4]) % MOD;
        curr = next
    }
    curr.into_iter().fold(0, |acc, v| (acc + v) % MOD)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(count_vowel_permutation(1), 5);
        assert_eq!(count_vowel_permutation(2), 10);
        assert_eq!(count_vowel_permutation(5), 68);
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
