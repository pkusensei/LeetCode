mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn pick_gifts(gifts: Vec<i32>, k: i32) -> i64 {
    let mut heap = std::collections::BinaryHeap::from(gifts);
    for _ in 0..k {
        let mut v = heap.pop().unwrap();
        v = (v as f64).sqrt().floor() as i32;
        heap.push(v);
    }
    heap.into_iter().map(i64::from).sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(pick_gifts([25, 64, 9, 4, 100].into(), 4), 29);
        assert_eq!(pick_gifts([1, 1, 1, 1].into(), 4), 4);
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
