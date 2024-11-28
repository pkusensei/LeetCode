mod dsu;
mod helper;
mod trie;

use std::collections::BTreeMap;

#[allow(unused_imports)]
use helper::*;

pub fn car_pooling(trips: &[[i32; 3]], capacity: i32) -> bool {
    let map = trips.iter().fold(BTreeMap::new(), |mut acc, v| {
        let (num, from, to) = (v[0], v[1], v[2]);
        *acc.entry(from).or_insert(0) += num;
        *acc.entry(to).or_insert(0) -= num;
        acc
    });
    let mut prefix = 0;
    for num in map.into_values() {
        prefix += num;
        if prefix > capacity {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!car_pooling(&[[2, 1, 5], [3, 3, 7]], 4));
        debug_assert!(car_pooling(&[[2, 1, 5], [3, 3, 7]], 5));
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
