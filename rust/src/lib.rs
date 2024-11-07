mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn three_sum_multi(arr: &[i32], target: i32) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let nums = arr
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0i64) += 1;
            acc
        });
    let mut res = 0;
    for (&n1, &c1) in nums.iter() {
        for (&n2, &c2) in nums.iter() {
            let n3 = target - n1 - n2;
            let Some(&c3) = nums.get(&n3) else {
                continue;
            };
            if n1 == n2 && n2 == n3 {
                res += c1 * (c1 - 1) * (c1 - 2) / 6 % MOD;
            }
            if n1 == n2 && n2 != n3 {
                res += c1 * (c1 - 1) / 2 * c3 % MOD;
            }
            if n1 < n2 && n2 < n3 {
                res += c1 * c2 * c3 % MOD;
            }
        }
    }
    (res % MOD) as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(three_sum_multi(&[1, 1, 2, 2, 3, 3, 4, 4, 5, 5], 8), 20);
        debug_assert_eq!(three_sum_multi(&[1, 1, 2, 2, 2, 2], 5), 12);
        debug_assert_eq!(three_sum_multi(&[2, 1, 3], 6), 1);
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
