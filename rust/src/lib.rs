mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

const MOD: usize = 1_000_000_007;

pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
    let [n, goal, k] = [n, goal, k].map(|v| v as usize);
    let mut dp = vec![vec![0; 1 + n]; 1 + goal];
    dp[0][0] = 1;
    for len in 1..=goal {
        for choice in 1..=n.min(len) {
            // choice is a new song
            // (choice-1) songs have been used => (n-(choice-1))
            dp[len][choice] = dp[len - 1][choice - 1] * (n - choice + 1) % MOD;
            if choice > k {
                // able to pick an a used song
                // the latest k songs are ineligible => (choice-k)
                dp[len][choice] += dp[len - 1][choice] * (choice - k) % MOD;
                dp[len][choice] %= MOD
            }
        }
    }
    dp[goal][n] as i32
}

fn combinatorics(n: i32, goal: i32, k: i32) -> i32 {
    let [n, goal, k] = [n, goal, k].map(|v| v as usize);
    let [factorials, inv_fact] = factorials(n);
    let mut sign = 1;
    let mut res = 0;
    for idx in (k..=n).rev() {
        let mut temp = mod_pow(idx - k, goal - k);
        temp = (temp * inv_fact[n - idx]) % MOD;
        temp = (temp * inv_fact[idx - k]) % MOD;
        res = (res + sign * temp as i64).rem_euclid(MOD as i64);
        sign *= -1;
    }
    ((factorials[n] * res as usize) % MOD) as i32
}

fn factorials(n: usize) -> [Vec<usize>; 2] {
    let [mut factorial, mut inv_fact] = [0, 1].map(|_| {
        let mut v = Vec::with_capacity(1 + n);
        v.push(1);
        v
    });
    for i in 1..=n {
        factorial.push((factorial[i - 1] * i) % MOD);
        inv_fact.push(mod_pow(factorial[i], MOD - 2));
    }
    [factorial, inv_fact]
}

fn mod_pow(mut base: usize, mut exp: usize) -> usize {
    let mut res = 1;
    while exp > 0 {
        if exp & 1 == 1 {
            res = (res * base) % MOD;
        }
        exp >>= 1;
        base = (base * base) % MOD;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(combinatorics(3, 3, 1), 6);
        debug_assert_eq!(combinatorics(2, 3, 0), 6);
        debug_assert_eq!(combinatorics(2, 3, 1), 2);
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
