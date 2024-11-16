mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn results_array(nums: &[i32], k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut res = Vec::with_capacity(n + 1 - k as usize);
    let mut prev = None;
    for win in nums.windows(k as usize) {
        if prev.is_some_and(|v| v + 1 == win[k as usize - 1])
            || win.windows(2).all(|w| w[0] + 1 == w[1])
        {
            let temp = win[k as usize - 1];
            prev = Some(temp);
            res.push(temp);
        } else {
            prev = None;
            res.push(-1);
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
        debug_assert_eq!(results_array(&[1, 2, 3, 4, 3, 2, 5], 3), [3, 4, -1, -1, -1]);
        debug_assert_eq!(results_array(&[2, 2, 2, 2, 2], 4), [-1, -1]);
        debug_assert_eq!(results_array(&[3, 2, 3, 2, 3, 2], 2), [-1, 3, -1, 3, -1]);
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
