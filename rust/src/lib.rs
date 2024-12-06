mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn make_array_increasing(arr1: &[i32], arr2: &mut [i32]) -> i32 {
    arr2.sort_unstable();
    let mut dp = HashMap::new();
    // let res = dfs(arr1, &arr2, 0, -1, &mut dp);
    // if res == arr2.len() {
    //     -1
    // } else {
    //     res as i32
    // }
    dp.insert(-1, 0);
    for &num in arr1.iter() {
        let mut next = HashMap::new();
        for (&prev, &count) in dp.iter() {
            if num > prev {
                let v = *next.get(&num).unwrap_or(&i32::MAX);
                next.insert(num, v.min(count));
            }
            let i = arr2.partition_point(|&v| v <= prev);
            if let Some(v) = arr2.get(i) {
                let val = *next.get(v).unwrap_or(&i32::MAX);
                next.insert(*v, val.min(1 + count));
            }
        }
        dp = next;
    }
    dp.into_values()
        .min()
        .map(|v| if v == i32::MAX { -1 } else { v })
        .unwrap_or(-1)
}

fn dfs(
    arr1: &[i32],
    arr2: &[i32],
    idx: usize,
    prev: i32,
    dp: &mut HashMap<(usize, i32), usize>,
) -> usize {
    if idx == arr1.len() {
        return 0;
    }
    if let Some(&v) = dp.get(&(idx, prev)) {
        return v;
    }
    let mut res = arr2.len();
    if arr1[idx] > prev {
        res = dfs(arr1, arr2, 1 + idx, arr1[idx], dp);
    }
    let i = arr2.partition_point(|&v| v <= prev);
    if let Some(&v) = arr2.get(i) {
        let temp = 1 + dfs(arr1, arr2, 1 + idx, v, dp);
        res = res.min(temp);
    }
    dp.insert((idx, prev), res);
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            make_array_increasing(&[1, 5, 3, 6, 7], &mut [1, 3, 2, 4]),
            1
        );
        assert_eq!(make_array_increasing(&[1, 5, 3, 6, 7], &mut [4, 3, 1]), 2);
        assert_eq!(
            make_array_increasing(&[1, 5, 3, 6, 7], &mut [1, 6, 3, 3]),
            -1
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            make_array_increasing(
                &[1, 28, 27, 14, 5, 16, 31, 2, 9, 4, 3, 22, 13, 24, 7, 10],
                &mut [12, 11, 30, 21, 0, 15, 18, 25, 20, 19, 6, 29, 8, 23, 26, 1, 28]
            ),
            13
        );
        assert_eq!(
            make_array_increasing(
                &[9, 18, 3, 8, 21, 6, 7, 2, 7, 28, 23, 16, 33, 2, 25, 14, 15],
                &mut [
                    13, 2, 15, 30, 31, 30, 9, 10, 7, 30, 31, 4, 33, 10, 25, 28, 19, 6, 15, 6, 19,
                    30, 25, 14, 7, 28, 23, 20, 1, 2, 25, 16
                ]
            ),
            14
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
