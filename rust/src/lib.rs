mod dsu;
mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn subarray_bitwise_ors(arr: &[i32]) -> i32 {
    let mut res: HashSet<i32> = HashSet::new();
    let mut prev = HashSet::new();
    // Suppose for numbers up until i-1
    // Their bitor results are in prev,
    // ... i-1, i, ..
    for &num in arr.iter() {
        // Now for number [i], the current set is
        // the union of [i] , and [i] bitor each number in result
        // since a bit set with bitor can never be unset by future bitors
        let curr: HashSet<_> = prev
            .iter()
            .map(|i| i | num)
            .chain(std::iter::once(num))
            .collect();
        prev = curr; // could be on the same line, but for clarity...
        res.extend(prev.iter());
    }
    res.len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(subarray_bitwise_ors(&[0]), 1);
        debug_assert_eq!(subarray_bitwise_ors(&[1, 1, 2]), 3);
        debug_assert_eq!(subarray_bitwise_ors(&[1, 2, 4]), 6);
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
