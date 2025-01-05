mod dsu;
mod helper;
mod trie;

use std::collections::{BinaryHeap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn min_set_size(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut heap: BinaryHeap<_> = arr
        .iter()
        .fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .map(|(num, count)| (count, num))
        .collect();
    let mut count = 0;
    let mut res = 0;
    while count < n / 2 {
        let Some((c, _v)) = heap.pop() else {
            break;
        };
        count += c;
        res += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(min_set_size(&[3, 3, 3, 3, 5, 5, 5, 2, 2, 7]), 2);
        assert_eq!(min_set_size(&[7, 7, 7, 7, 7, 7]), 1);
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
