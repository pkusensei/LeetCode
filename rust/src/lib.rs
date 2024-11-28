mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
    match label.ilog2() {
        0 => vec![1],
        1 => vec![1, label],
        n => {
            let min = 2i32.pow(n - 1);
            let max = 2i32.pow(n) - 1;
            let ideal = label / 2;
            let parent = min + max - ideal;
            let mut res = path_in_zig_zag_tree(parent);
            res.push(label);
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(path_in_zig_zag_tree(14), [1, 3, 4, 14]);
        debug_assert_eq!(path_in_zig_zag_tree(26), [1, 2, 6, 10, 26]);
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
