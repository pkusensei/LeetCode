mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_lhs(nums: &[i32]) -> i32 {
    nums.iter()
        .fold(std::collections::BTreeMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
        .into_iter()
        .collect::<Vec<_>>()
        .windows(2)
        .filter_map(|w| {
            if w[1].0 - w[0].0 == 1 {
                Some(w[1].1 + w[0].1)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_lhs(&[1, 3, 2, 2, 5, 2, 3, 7]), 5);
        debug_assert_eq!(find_lhs(&[1, 2, 3, 4]), 2);
        debug_assert_eq!(find_lhs(&[1, 1, 1, 1]), 0);
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
}
