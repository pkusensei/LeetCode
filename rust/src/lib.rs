mod dsu;
mod helper;
mod trie;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn advantage_count(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let n = nums1.len();
    let mut heap: BinaryHeap<_> = nums1.iter().copied().map(Reverse).collect();
    let mut nums2: Vec<_> = nums2.iter().copied().enumerate().collect();
    nums2.sort_unstable_by_key(|&(_, v)| v);
    let mut res = vec![-1; n];
    let mut temp = vec![];
    for (i, num) in nums2.into_iter() {
        while let Some(Reverse(v)) = heap.pop() {
            if v <= num {
                temp.push(v);
            } else {
                res[i] = v;
                break;
            }
        }
        heap.extend(temp.into_iter().map(Reverse));
        temp = vec![];
    }
    for v in res.iter_mut().filter(|v| **v < 0) {
        let Some(x) = heap.pop() else { break };
        *v = x.0;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            advantage_count(&[2, 7, 11, 15], &[1, 10, 4, 11]),
            [2, 11, 7, 15]
        );
        debug_assert_eq!(
            advantage_count(&[12, 24, 8, 32], &[13, 25, 32, 11]),
            [24, 32, 8, 12]
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
