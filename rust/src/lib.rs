mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_length_of_lcis(nums: &[i32]) -> i32 {
    let (mut res, mut curr) = (1, 1);
    for w in nums.windows(2) {
        if w[0] < w[1] {
            curr += 1;
            res = res.max(curr);
        } else {
            curr = 1;
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
        debug_assert_eq!(find_length_of_lcis(&[1, 3, 5, 4, 7]), 3);
        debug_assert_eq!(find_length_of_lcis(&[2, 2, 2, 2, 2]), 1);
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
