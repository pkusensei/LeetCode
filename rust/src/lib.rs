mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn split_array_same_average(nums: &[i32]) -> bool {
    let n = nums.len();
    if n == 1 {
        return false;
    }
    let sum: i32 = nums.iter().sum();
    if !(1..=n / 2).any(|i| sum as usize * i % n == 0) {
        return false;
    }
    let mut subsets = vec![HashSet::new(); 1 + n / 2];
    subsets[0].insert(0);
    for num in nums.iter() {
        for idx in (1..=n / 2).rev() {
            let a: Vec<_> = subsets[idx - 1].iter().map(|&v| num + v).collect();
            subsets[idx].extend(a);
        }
    }
    for (idx, subset) in subsets.iter().enumerate().skip(1) {
        if sum as usize * idx % n == 0 && subset.contains(&(sum * idx as i32 / n as i32)) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(split_array_same_average(&[1, 2, 3, 4, 5, 6, 7, 8]));
        debug_assert!(!split_array_same_average(&[3, 1]));
    }

    #[test]
    fn test() {
        debug_assert!(!split_array_same_average(&[6, 8, 18, 3, 1]));
        debug_assert!(!split_array_same_average(&[
            60, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
            30, 30, 30, 30, 30, 30, 30, 30
        ]));
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
}
