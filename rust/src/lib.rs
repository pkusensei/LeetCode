mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn last_stone_weight_ii(stones: &[i32]) -> i32 {
    let mut dp = std::collections::HashSet::from([0]);
    for num in stones.iter() {
        dp = dp
            .into_iter()
            .flat_map(|v| [v + num, (v - num).abs()])
            .collect();
    }
    dp.into_iter().min().unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(last_stone_weight_ii(&[2, 7, 4, 1, 8, 1]), 1);
        debug_assert_eq!(last_stone_weight_ii(&[31, 26, 33, 21, 40]), 5);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            last_stone_weight_ii(&[
                89, 23, 100, 93, 82, 98, 91, 85, 33, 95, 72, 98, 63, 46, 17, 91, 92, 72, 77, 79,
                99, 96, 55, 72, 24, 98, 79, 93, 88, 92
            ]),
            0
        );
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
