mod dsu;
mod helper;
mod trie;

use std::collections::{BTreeMap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn maximum_beauty(items: &[[i32; 2]], queries: &[i32]) -> Vec<i32> {
    let mut prefix: BTreeMap<i32, i32> = BTreeMap::new();
    for &[key, val] in items.iter() {
        let v = prefix.entry(key).or_insert(0);
        *v = (*v).max(val);
    }
    let mut curr = *prefix.values().next().unwrap_or(&1);
    for v in prefix.values_mut() {
        curr = curr.max(*v);
        *v = curr;
    }
    let mut seen = HashMap::new();
    let mut res = Vec::with_capacity(queries.len());
    for &num in queries.iter() {
        if let Some(&v) = seen.get(&num) {
            res.push(v);
        } else {
            let v = prefix
                .range(..=num)
                .next_back()
                .map(|(_, &v)| v)
                .unwrap_or(0);
            res.push(v);
            seen.insert(num, v);
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
        debug_assert_eq!(
            maximum_beauty(
                &[[1, 2], [3, 2], [2, 4], [5, 6], [3, 5]],
                &[1, 2, 3, 4, 5, 6]
            ),
            [2, 4, 5, 5, 6, 6]
        );
        debug_assert_eq!(maximum_beauty(&[[1, 2], [1, 2], [1, 3], [1, 4]], &[1]), [4]);
        debug_assert_eq!(maximum_beauty(&[[10, 1000]], &[5]), [0]);
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
