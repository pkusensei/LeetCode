mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(position: &mut [i32], m: i32) -> i32 {
    let n = position.len();
    position.sort_unstable();
    let mut right = position[n - 1] - position[0];
    let mut left = position.windows(2).map(|w| w[1] - w[0]).min().unwrap_or(1);
    let mut res = 0;
    while left <= right {
        let mid = (right - left) / 2 + left;
        if count(position, mid) >= m {
            res = mid;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    res
}

fn count(pos: &[i32], mid: i32) -> i32 {
    let mut idx = 0;
    let mut res = 0;
    while idx < pos.len() {
        res += 1;
        let curr = pos[idx];
        idx = pos.partition_point(|&v| v < curr + mid);
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_distance(&mut [1, 2, 3, 4, 7], 3), 3);
        assert_eq!(max_distance(&mut [5, 4, 3, 2, 1, 1000000000], 2), 999999999);
    }

    #[test]
    fn test() {
        assert_eq!(max_distance(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 4), 3);
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
