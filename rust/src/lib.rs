mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn k_inverse_pairs(n: i32, k: i32) -> i32 {
    const MOD: i32 = 1_000_000_007;
    let (n, k) = (n as usize, k as usize);
    // let (mut prev, mut curr) = (vec![0; 1 + k], vec![0; 1 + k]);
    // prev[0] = 1;
    // for i1 in 1..=n {
    //     for i2 in 0..=k {
    //         for i3 in 0..=i2.min(i1 - 1) {
    //             curr[i2] = (curr[i2] + prev[i2 - i3]) % MOD;
    //         }
    //     }
    //     prev = curr;
    //     curr = vec![0; 1 + k];
    // }
    // prev[k]
    let mut dp = vec![0; 1 + k];
    dp[0] = 1;

    for i in 2..=n {
        for j in 1..=k {
            dp[j] = (dp[j] + dp[j - 1]) % MOD;
        }
        for j in (i..=k).rev() {
            dp[j] = (dp[j] - dp[j - i]).rem_euclid(MOD);
        }
    }
    dp[k]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(k_inverse_pairs(3, 0), 1);
        debug_assert_eq!(k_inverse_pairs(3, 1), 2);
    }

    #[test]
    fn test() {
        debug_assert_eq!(k_inverse_pairs(3, 2), 2);
        debug_assert_eq!(k_inverse_pairs(1000, 1000), 663677020);
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
