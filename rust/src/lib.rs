mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn smallest_range(nums: &[&[i32]]) -> Vec<i32> {
    let n = nums.len();
    let mut nis: Vec<_> = nums
        .iter()
        .enumerate()
        .flat_map(|(idx, v)| v.iter().map(move |&num| (num, idx)))
        .collect();
    nis.sort_unstable_by_key(|p| p.0);
    let mut nis_i = 0;
    let mut res = [0, nis.last().unwrap().0];
    let mut map = HashMap::new();
    for &(right, nums_i) in nis.iter() {
        *map.entry(nums_i).or_insert(0) += 1;
        while map.len() == n && map[&nis[nis_i].1] > 1 {
            map.entry(nis[nis_i].1).and_modify(|c| *c -= 1);
            nis_i += 1;
        }
        let left = nis[nis_i].0;
        if map.len() == n && right - left < res[1] - res[0] {
            res = [left, right];
        }
    }
    res.to_vec()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            smallest_range(&[&[4, 10, 15, 24, 26], &[0, 9, 12, 20], &[5, 18, 22, 30]]),
            [20, 24]
        );
        debug_assert_eq!(
            smallest_range(&[&[1, 2, 3], &[1, 2, 3], &[1, 2, 3]]),
            [1, 1]
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
}
