mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn kth_factor(n: i32, k: i32) -> i32 {
    let mid = f64::from(n).sqrt().floor() as i32;
    let mut queue = std::collections::VecDeque::new();
    for v in (1..=mid).rev() {
        if v == mid && mid.pow(2) == n {
            queue.push_back(mid);
        }
        if n % v == 0 {
            queue.push_front(v);
            queue.push_back(n / v);
        }
    }
    queue.into_iter().nth(k as usize - 1).unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
