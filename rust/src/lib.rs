mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn increasing_triplet(nums: &[i32]) -> bool {
    if nums.len() < 3 {
        return false;
    }
    let (mut n1, mut n2) = (i32::MAX, i32::MAX);
    for &num in nums.iter() {
        if num <= n1 {
            n1 = num;
        } else if num <= n2 {
            n2 = num
        } else {
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
        debug_assert!(increasing_triplet(&[1, 2, 3, 4, 5]));
        debug_assert!(!increasing_triplet(&[5, 4, 3, 2, 1]));
        debug_assert!(increasing_triplet(&[2, 1, 5, 0, 4, 6]));
    }

    #[test]
    fn test() {
        debug_assert!(!increasing_triplet(&[0, 4, 2, 1, 0, -1, -3]))
    }

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
