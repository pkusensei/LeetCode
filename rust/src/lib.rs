mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
    if bound < 2 {
        return vec![];
    }
    match (x, y) {
        (1, 1) => vec![2],
        (1, num) | (num, 1) => {
            debug_assert!(num > 1);
            let mut p = 0;
            let mut res = HashSet::new();
            while num.pow(p) < bound {
                res.insert(1 + num.pow(p));
                p += 1;
            }
            res.into_iter().collect()
        }
        _ => {
            let mut res = HashSet::new();
            let px = bound.ilog(x);
            let py = bound.ilog(y);
            for ix in 0..=px {
                for iy in 0..=py {
                    let temp = x.pow(ix) + y.pow(iy);
                    if temp <= bound {
                        res.insert(temp);
                    }
                }
            }
            res.into_iter().collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(powerful_integers(2, 3, 10), [2, 3, 4, 5, 7, 9, 10]);
        sort_eq(powerful_integers(3, 5, 15), [2, 4, 6, 8, 10, 14]);
    }

    #[test]
    fn test() {
        debug_assert!(powerful_integers(1, 1, 0).is_empty());
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
