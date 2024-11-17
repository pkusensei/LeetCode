mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn broken_calc(start_value: i32, target: i32) -> i32 {
    dfs(start_value, target)
}

fn dfs(start: i32, target: i32) -> i32 {
    if start >= target {
        start - target
    } else if target & 1 == 1 {
        1 + dfs(start, 1 + target)
    } else {
        1 + dfs(start, target / 2)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(broken_calc(2, 3), 2);
        debug_assert_eq!(broken_calc(5, 8), 2);
        debug_assert_eq!(broken_calc(3, 10), 3);
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
