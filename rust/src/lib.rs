mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn can_reorder_doubled(arr: &mut [i32]) -> bool {
    arr.sort_unstable();
    let mut counts = arr.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    for &num in arr.iter() {
        if counts.get(&num).is_some_and(|&v| v > 0)
            && counts.get(&(2 * num)).is_some_and(|&v| v > 0)
        {
            counts.entry(num).and_modify(|v| *v -= 1);
            counts.entry(2 * num).and_modify(|v| *v -= 1);
        }
    }
    counts.into_values().all(|v| v == 0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(!can_reorder_doubled(&mut [3, 1, 3, 6]));
        debug_assert!(!can_reorder_doubled(&mut [2, 1, 2, 6]));
        debug_assert!(can_reorder_doubled(&mut [4, -2, 2, -4]));
    }

    #[test]
    fn test() {
        debug_assert!(can_reorder_doubled(&mut [2, 4, 0, 0, 8, 1]));
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
