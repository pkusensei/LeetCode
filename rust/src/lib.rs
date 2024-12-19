mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    // 4x + 2y = tomato
    // x + y = cheese
    // 2x = tomato - 2cheese
    let x = (tomato_slices - 2 * cheese_slices) / 2;
    let y = cheese_slices - x;
    if x >= 0 && y >= 0 && 4 * x + 2 * y == tomato_slices {
        vec![x, y]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(num_of_burgers(16, 7), [1, 6]);
        assert!(num_of_burgers(17, 4).is_empty());
        assert!(num_of_burgers(4, 17).is_empty());
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
