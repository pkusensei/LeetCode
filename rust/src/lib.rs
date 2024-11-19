mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_pairs_divisible_by60(time: &[i32]) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut res = 0;
    for &num in time.iter() {
        let k = num % 60;
        let m = if k > 0 { 60 - k } else { 0 };
        if let Some(v) = map.get(&m) {
            res += v
        }
        *map.entry(k).or_insert(0) += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_pairs_divisible_by60(&[30, 20, 150, 100, 40]), 3);
        debug_assert_eq!(num_pairs_divisible_by60(&[60, 60, 60]), 3);
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
