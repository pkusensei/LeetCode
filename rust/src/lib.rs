mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn kth_smallest(mat: &[&[i32]], k: i32) -> i32 {
    let mut heap: BinaryHeap<_> = mat[0].iter().map(|&v| Reverse(v)).collect();
    for row in mat.iter().skip(1) {
        let mut next: BinaryHeap<_> = heap
            .into_iter()
            .flat_map(|Reverse(prev)| row.iter().map(move |v| Reverse(prev + v)))
            .collect();
        heap = BinaryHeap::with_capacity(k as usize);
        let mut k = k;
        while let Some(v) = next.pop() {
            heap.push(v);
            k -= 1;
            if 0 == k {
                break;
            }
        }
    }
    let mut res = 0;
    let mut k = k;
    while k > 0 {
        let Some(Reverse(v)) = heap.pop() else {
            return res;
        };
        res = v;
        k -= 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(kth_smallest(&[&[1, 10, 10], &[1, 4, 5], &[2, 3, 6]], 7), 9);
        assert_eq!(kth_smallest(&[&[1, 3, 11], &[2, 4, 6]], 5), 7);
        assert_eq!(kth_smallest(&[&[1, 3, 11], &[2, 4, 6]], 9), 17);
    }

    #[test]
    fn test() {
        assert_eq!(
            kth_smallest(
                &[
                    &[6, 9, 32, 34, 43, 45],
                    &[5, 10, 18, 21, 40, 42],
                    &[15, 16, 22, 33, 50, 50],
                    &[14, 27, 29, 31, 33, 39],
                    &[1, 11, 19, 25, 45, 50],
                    &[3, 9, 17, 19, 26, 30],
                    &[3, 14, 26, 29, 41, 43],
                    &[12, 23, 25, 26, 40, 46]
                ],
                47
            ),
            75
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
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
