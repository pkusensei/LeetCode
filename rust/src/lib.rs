mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_three_parts_equal_sum(arr: &[i32]) -> bool {
    let sum: i32 = arr.iter().sum();
    if sum % 3 > 0 {
        return false;
    }
    let sum = sum / 3;
    let mut curr = 0;
    let mut count = 3;
    let mut idx = 0;
    for (i, &num) in arr.iter().enumerate() {
        curr += num;
        if curr == sum {
            curr = 0;
            count -= 1;
        }
        if count == 1 {
            idx = 1 + i;
            break;
        }
    }
    count == 1 && idx < arr.len() && arr[idx..].iter().sum::<i32>() == sum
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(can_three_parts_equal_sum(&[
            0, 2, 1, -6, 6, -7, 9, 1, 2, 0, 1
        ]));
        debug_assert!(!can_three_parts_equal_sum(&[
            0, 2, 1, -6, 6, 7, 9, -1, 2, 0, 1
        ]));
        debug_assert!(can_three_parts_equal_sum(&[3, 3, 6, 5, -2, 2, 5, 1, -9, 4]));
    }

    #[test]
    fn test() {
        debug_assert!(can_three_parts_equal_sum(&[0, 0, 0, 0, 0]));
        debug_assert!(!can_three_parts_equal_sum(&[1, -1, 1, -1]));
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
