mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_equiv_domino_pairs(dominoes: &[[i32; 2]]) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut res = 0;
    for d in dominoes {
        let a = d[0].min(d[1]);
        let b = d[0].max(d[1]);
        if let Some(v) = map.get_mut(&[a, b]) {
            res += *v;
            *v += 1;
        } else {
            map.insert([a, b], 1);
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
        debug_assert_eq!(num_equiv_domino_pairs(&[[1, 2], [2, 1], [3, 4], [5, 6]]), 1);
        debug_assert_eq!(
            num_equiv_domino_pairs(&[[1, 2], [1, 2], [1, 1], [1, 2], [2, 2]]),
            3
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
