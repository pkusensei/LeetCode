mod helper;

use std::{collections::HashMap, iter};

#[allow(unused_imports)]
use helper::*;

pub fn intersect(nums1: &[i32], nums2: &[i32]) -> Vec<i32> {
    let [m1, m2] = [nums1, nums2].map(|nums| {
        nums.iter().fold(HashMap::new(), |mut acc, &num| {
            *acc.entry(num).or_insert(0) += 1;
            acc
        })
    });
    let mut res = vec![];
    for (k, v1) in m1.into_iter() {
        if let Some(&v2) = m2.get(&k) {
            res.extend(iter::repeat(k).take(v1.min(v2)))
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
        sort_eq(intersect(&[1, 2, 2, 1], &[2, 2]), [2, 2]);
        sort_eq(intersect(&[4, 9, 5], &[9, 4, 9, 8, 4]), [4, 9]);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
