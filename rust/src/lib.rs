mod dsu;
mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn mincost_to_hire_workers(quality: &[i32], wage: &[i32], k: i32) -> f64 {
    let mut pairs: Vec<_> = quality
        .iter()
        .zip(wage.iter())
        .map(|(&q, &w)| (q, f64::from(w) / f64::from(q)))
        .collect();
    pairs.sort_unstable_by(|a, b| a.1.total_cmp(&b.1));
    // want lowest wage/quality ratio
    // and keep lowest quality in heap
    let mut heap = BinaryHeap::new();
    let mut res = f64::MAX;
    let mut quality = 0;
    for curr in pairs.into_iter() {
        heap.push(curr.0);
        quality += curr.0;
        if heap.len() > k as usize {
            let Some(top) = heap.pop() else {
                continue;
            };
            quality -= top;
        }
        if heap.len() == k as usize {
            res = res.min(f64::from(quality) * curr.1);
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
        float_eq(
            mincost_to_hire_workers(&[10, 20, 5], &[70, 50, 30], 2),
            105.0,
        );
        float_eq(
            mincost_to_hire_workers(&[3, 1, 10, 10, 1], &[4, 8, 2, 2, 7], 3),
            30.66667,
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
    fn float_eq<T1, T2>(a: T1, b: T2) -> bool
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP
    }
}
