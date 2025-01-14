mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(card_points: &[i32], k: i32) -> i32 {
    let (n, k) = (card_points.len(), k as usize);
    let sum: i32 = card_points.iter().sum();
    if n <= k {
        return sum;
    }
    let len = n - k;
    let mut window: i32 = card_points[..len].iter().sum();
    let mut res = sum - window;
    for idx in 1..=n - len {
        window -= card_points[idx - 1];
        window += card_points[idx + len - 1];
        res = res.max(sum - window);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_score(&[1, 2, 3, 4, 5, 6, 1], 3), 12);
        assert_eq!(max_score(&[2, 2, 2], 2), 4);
        assert_eq!(max_score(&[9, 7, 7, 9, 7, 7, 9], 7), 55);
    }

    #[test]
    fn test() {
        assert_eq!(max_score(&[100, 40, 17, 9, 73, 75], 3), 248);
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
