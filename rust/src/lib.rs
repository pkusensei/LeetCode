mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn distribute_candies(mut candies: i32, num_people: i32) -> Vec<i32> {
    let n = num_people as usize;
    let mut res = vec![0; n];
    let mut idx = 0;
    while candies > 0 {
        res[idx % n] += (1 + idx as i32).min(candies);
        idx += 1;
        candies -= idx as i32;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(distribute_candies(7, 4), [1, 2, 3, 1]);
        debug_assert_eq!(distribute_candies(10, 3), [5, 2, 3]);
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
