mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_abs_difference(arr: &mut [i32]) -> Vec<Vec<i32>> {
    arr.sort_unstable();
    let mut delta = i32::MAX;
    let mut res = vec![];
    for w in arr.windows(2) {
        let d = w[1] - w[0];
        match d.cmp(&delta) {
            std::cmp::Ordering::Less => {
                delta = d;
                res = vec![vec![w[0], w[1]]];
            }
            std::cmp::Ordering::Equal => {
                res.push(vec![w[0], w[1]]);
            }
            std::cmp::Ordering::Greater => (),
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
        assert_eq!(
            minimum_abs_difference(&mut [4, 2, 1, 3]),
            [[1, 2], [2, 3], [3, 4]]
        );
        assert_eq!(minimum_abs_difference(&mut [1, 3, 6, 10, 15]), [[1, 3]]);
        assert_eq!(
            minimum_abs_difference(&mut [3, 8, -10, 23, 19, -4, -14, 27]),
            [[-14, -10], [19, 23], [23, 27]]
        );
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
