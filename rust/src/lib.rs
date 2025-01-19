mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn winner_square_game(n: i32) -> bool {
    let n = n as usize;
    let mut dp = vec![false; 1 + n];
    for i in 1..=n {
        let mut sqrt = 1usize;
        while sqrt.pow(2) <= i {
            if !dp[i - sqrt.pow(2)] {
                dp[i] = true;
                break;
            }
            sqrt += 1;
        }
    }
    dp[n]
    // dfs(n, &mut vec![-1; 1 + n])
}

fn dfs(n: usize, memo: &mut [i8]) -> bool {
    if is_square(n) {
        return true;
    }
    if memo[n] > -1 {
        return memo[n] == 1;
    }
    let mut res = false;
    for i in (1..n).rev().filter(|&i| is_square(i)) {
        if !dfs(n - i, memo) {
            res = true;
            break;
        }
    }
    memo[n] = if res { 1 } else { 0 };
    res
}

fn is_square(n: usize) -> bool {
    let sqrt = (n as f64).sqrt().floor() as usize;
    sqrt.pow(2) == n
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(winner_square_game(1));
        assert!(!winner_square_game(2));
        assert!(winner_square_game(4));
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
