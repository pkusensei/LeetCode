mod dsu;
mod helper;
mod trie;

use std::collections::{BinaryHeap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn largest_vals_from_labels(
    values: &[i32],
    labels: &[i32],
    num_wanted: i32,
    use_limit: i32,
) -> i32 {
    let mut heap: BinaryHeap<_> = values
        .iter()
        .zip(labels.iter())
        .map(|(val, lab)| (*val, *lab))
        .collect();
    let mut res = 0;
    let mut count = HashMap::new();
    let mut curr = num_wanted;
    while curr > 0 {
        let Some((val, lab)) = heap.pop() else {
            break;
        };
        *count.entry(lab).or_insert(0) += 1;
        if count[&lab] > use_limit {
            continue;
        }
        res += val;
        curr -= 1;
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
            largest_vals_from_labels(&[5, 4, 3, 2, 1], &[1, 1, 2, 2, 3], 3, 1),
            9
        );
        debug_assert_eq!(
            largest_vals_from_labels(&[5, 4, 3, 2, 1], &[1, 3, 3, 3, 2], 3, 2),
            12
        );
        debug_assert_eq!(
            largest_vals_from_labels(&[9, 8, 8, 7, 6], &[0, 0, 0, 1, 1], 3, 1),
            16
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
