mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn longest_common_prefix(arr1: &[i32], arr2: &[i32]) -> i32 {
    let prefix: HashSet<_> = arr1.iter().flat_map(|&num| to_prefix(num)).collect();
    let mut res = 0;
    for n in arr2.iter().flat_map(|&num| to_prefix(num)) {
        if prefix.contains(&n) {
            res = res.max(1 + n.ilog10())
        }
    }
    res as _
}

fn to_prefix(num: i32) -> impl Iterator<Item = i32> {
    std::iter::successors(Some(num), |n| if n / 10 > 0 { Some(n / 10) } else { None })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_common_prefix(&[1, 10, 100], &[1000]), 3);
        debug_assert_eq!(longest_common_prefix(&[1, 2, 3], &[4, 4, 4]), 0);
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
