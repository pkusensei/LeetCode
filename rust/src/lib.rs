mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn duplicate_zeros(arr: &mut [i32]) {
    let mut dups = 0;
    let mut len = arr.len();
    let mut idx = 0;
    while idx < len - dups {
        if arr[idx] == 0 {
            if idx + 1 == len - dups {
                // Each 0 should be duped
                // But this one sitting on the last slot avoids duplication
                // Hence it's only moved over
                arr[len - 1] = 0;
                len -= 1;
                break;
            }
            dups += 1;
        }
        idx += 1;
    }
    for idx in (0..len - dups).rev() {
        arr[idx + dups] = arr[idx];
        if arr[idx] == 0 {
            dups -= 1;
            arr[idx + dups] = 0;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        {
            let arr = &mut [1, 0, 2, 3, 0, 4, 5, 0];
            duplicate_zeros(arr);
            debug_assert_eq!(*arr, [1, 0, 0, 2, 3, 0, 0, 4]);
        }
        {
            let arr = &mut [1, 2, 3];
            duplicate_zeros(arr);
            debug_assert_eq!(*arr, [1, 2, 3]);
        }
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
