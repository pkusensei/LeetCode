mod dsu;
mod helper;
mod trie;

use std::{collections::HashMap, iter};

#[allow(unused_imports)]
use helper::*;

pub fn relative_sort_array(arr1: &[i32], arr2: &[i32]) -> Vec<i32> {
    let mut count = arr1.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    let mut res = Vec::with_capacity(arr1.len());
    for k in arr2.iter() {
        let Some(v) = count.remove(k) else {
            continue;
        };
        res.extend(iter::repeat(*k).take(v));
    }
    let mut rest: Vec<_> = count
        .into_iter()
        .flat_map(|(k, v)| iter::repeat(k).take(v))
        .collect();
    rest.sort_unstable();
    res.extend(rest);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            relative_sort_array(&[2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19], &[2, 1, 4, 3, 9, 6]),
            [2, 2, 2, 1, 4, 3, 3, 9, 6, 7, 19]
        );
        debug_assert_eq!(
            relative_sort_array(&[28, 6, 22, 8, 44, 17], &[22, 28, 8, 6]),
            [22, 28, 8, 6, 17, 44]
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
