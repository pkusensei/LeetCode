mod dsu;
mod helper;
mod trie;

use std::collections::{BinaryHeap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn rearrange_barcodes(barcodes: &[i32]) -> Vec<i32> {
    let mut heap: BinaryHeap<_> = barcodes
        .iter()
        .fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .map(|(num, count)| (count, num))
        .collect();
    let mut res = Vec::with_capacity(barcodes.len());
    while let Some((count, num)) = heap.pop() {
        if res.last().is_some_and(|&v| v == num) {
            let Some((c, n)) = heap.pop() else {
                break;
            };
            res.push(n);
            if c > 1 {
                heap.push((c - 1, n));
            }
            heap.push((count, num));
        } else {
            res.push(num);
            if count > 1 {
                heap.push((count - 1, num));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(rearrange_barcodes(&[1, 1, 1, 2, 2, 2]), [2, 1, 2, 1, 2, 1]);
        debug_assert_eq!(
            rearrange_barcodes(&[1, 1, 1, 1, 2, 2, 3, 3]),
            [1, 3, 1, 2, 1, 3, 2, 1]
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
