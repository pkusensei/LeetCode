mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn prefixes_div_by5(nums: &[i32]) -> Vec<bool> {
    let mut res = Vec::with_capacity(nums.len());
    let mut curr = 0;
    for &num in nums.iter() {
        curr = (curr << 1) | num;
        curr %= 5;
        res.push(curr == 0);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(prefixes_div_by5(&[0, 1, 1]), [true, false, false]);
        debug_assert_eq!(prefixes_div_by5(&[1, 1, 1]), [false, false, false]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            prefixes_div_by5(&[
                1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0,
                0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1
            ]),
            [
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, true, false, false, true, true,
                true, true, false
            ]
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
