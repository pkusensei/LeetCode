mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_satisfaction(satisfaction: &mut [i32]) -> i32 {
    satisfaction.sort_unstable();
    let i = satisfaction.partition_point(|&v| v < 0);
    if i == satisfaction.len() {
        return 0;
    }
    let mut delta: i32 = satisfaction[i..].iter().sum();
    let mut res: i32 = satisfaction[i..]
        .iter()
        .enumerate()
        .map(|(i, v)| (1 + i as i32) * v)
        .sum();
    for &num in satisfaction[..i].iter().rev() {
        delta += num;
        res = res.max(res + delta);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_satisfaction(&mut [-1, -8, 0, 5, -9]), 14);
        assert_eq!(max_satisfaction(&mut [4, 3, 2]), 20);
        assert_eq!(max_satisfaction(&mut [-1, -4, -5]), 0);
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
