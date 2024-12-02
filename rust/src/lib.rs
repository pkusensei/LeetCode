mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn moves_to_make_zigzag(nums: &[i32]) -> i32 {
    let [mut a, mut b] = [0; 2];
    for (i, &num) in nums.iter().enumerate() {
        let left = if i > 0 { nums[i - 1] } else { 1001 };
        let right = *nums.get(1 + i).unwrap_or(&1001);
        let min = left.min(right);
        if num >= min {
            if i & 1 == 0 {
                a += num - (min - 1);
            } else {
                b += num - (min - 1);
            }
        }
    }
    a.min(b) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(moves_to_make_zigzag(&[1, 2, 3]), 2);
        assert_eq!(moves_to_make_zigzag(&[9, 6, 1, 6, 2]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(moves_to_make_zigzag(&[2, 1, 2]), 0);
        assert_eq!(moves_to_make_zigzag(&[2, 7, 10, 9, 8, 9]), 4);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
