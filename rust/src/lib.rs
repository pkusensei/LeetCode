mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_duplicates(mut nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];
    let n = nums.len();
    if nums.len() < 2 {
        return res;
    }
    for idx in 0..n {
        let num = nums[idx];
        let i = num.unsigned_abs() as usize - 1;
        if nums[i] > 0 {
            nums[i] *= -1;
        } else {
            res.push(if num < 0 { -num } else { num });
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
        debug_assert_eq!(find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]), [2, 3]);
        debug_assert_eq!(find_duplicates(vec![1, 1, 2]), [1]);
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
