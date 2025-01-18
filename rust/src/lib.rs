mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn avoid_flood(rains: &[i32]) -> Vec<i32> {
    let mut res = vec![-1; rains.len()];
    let mut dry = BTreeSet::new();
    let mut wet = HashMap::new();
    for (idx, &num) in rains.iter().enumerate() {
        if num == 0 {
            dry.insert(idx);
        } else if let Some(prev) = wet.get_mut(&num) {
            let Some(&i) = dry.range(1 + *prev..).next() else {
                return vec![];
            };
            res[i] = num;
            dry.remove(&i);
            *prev = idx;
        } else {
            wet.insert(num, idx);
        }
    }
    for d in dry {
        res[d] = 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(avoid_flood(&[1, 2, 3, 4]), [-1; 4]);
        assert_eq!(avoid_flood(&[1, 2, 0, 0, 2, 1]), [-1, -1, 2, 1, -1, -1]);
        assert!(avoid_flood(&[1, 2, 0, 1, 2]).is_empty());
    }

    #[test]
    fn test() {
        assert_eq!(
            avoid_flood(&[1, 0, 2, 0, 3, 0, 2, 0, 0, 0, 1, 2, 3]),
            [-1, 1, -1, 2, -1, 3, -1, 2, 1, 1, -1, -1, -1]
        );
        assert!(avoid_flood(&[0, 1, 1]).is_empty());
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
