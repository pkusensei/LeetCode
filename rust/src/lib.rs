mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    nums.windows(k as usize)
        .map(|w| f64::from(w.iter().sum::<i32>()) / f64::from(k))
        .max_by(|a, b| a.total_cmp(b))
        .unwrap()
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
}
