mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
    let mut nums = [a, b, c];
    nums.sort_unstable();
    let [a, b, c] = nums;
    if a + 2 == c {
        vec![0, 0]
    } else if b - a <= 2 || c - b <= 2 {
        vec![1, c - a - 2]
    } else {
        vec![2, c - a - 2]
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_moves_stones(1, 2, 5), [1, 2]);
        debug_assert_eq!(num_moves_stones(4, 3, 2), [0, 0]);
        debug_assert_eq!(num_moves_stones(3, 5, 1), [1, 2]);
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
