mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_possible(target: Vec<i32>) -> bool {
    let mut sum: i64 = target.iter().map(|&v| i64::from(v)).sum();
    let mut heap: std::collections::BinaryHeap<_> = target.into_iter().map(i64::from).collect();
    while let Some(max) = heap.pop() {
        if max == 1 {
            return true;
        }
        let mod_ = sum - max;
        if max <= mod_ || mod_ == 0 {
            return false;
        }
        let val = max % mod_;
        if val == 0 && mod_ != 1 {
            return false;
        }
        sum = sum - max + val;
        heap.push(val);
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(is_possible(vec![9, 3, 5]));
        assert!(!is_possible(vec![1, 1, 1, 2]));
        assert!(is_possible(vec![8, 5]));
    }

    #[test]
    fn test() {
        assert!(!is_possible(vec![2, 900000002]));
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
