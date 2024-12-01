mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_abs_val_expr(arr1: &[i32], arr2: &[i32]) -> i32 {
    let n = arr1.len();
    let mut res = 0;
    for f in [f1, f2, f3, f4] {
        let temp = (0..n).map(|idx| f(arr1, arr2, idx)).max().unwrap()
            - (0..n).map(|idx| f(arr1, arr2, idx)).min().unwrap();
        res = res.max(temp);
    }
    res
}

fn f1(arr1: &[i32], arr2: &[i32], idx: usize) -> i32 {
    arr1[idx] + arr2[idx] + idx as i32
}

fn f2(arr1: &[i32], arr2: &[i32], idx: usize) -> i32 {
    arr1[idx] - arr2[idx] + idx as i32
}

fn f3(arr1: &[i32], arr2: &[i32], idx: usize) -> i32 {
    -arr1[idx] + arr2[idx] + idx as i32
}

fn f4(arr1: &[i32], arr2: &[i32], idx: usize) -> i32 {
    -arr1[idx] - arr2[idx] + idx as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!((max_abs_val_expr(&[1, 2, 3, 4], &[-1, 4, 5, 6])), 13);
        debug_assert_eq!(
            max_abs_val_expr(&[1, -2, -5, 0, 10], &[0, -2, -1, -7, -4]),
            20
        );
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
