mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_max_length(nums: &[i32]) -> i32 {
    let mut seen = HashMap::from([(0, -1)]);
    let mut prev = 0;
    let mut res = 0;
    for (i, &num) in nums.iter().enumerate() {
        prev += if num == 1 { 1 } else { -1 };
        if let Some(&v) = seen.get(&prev) {
            res = res.max(i as i32 - v)
        } else {
            seen.insert(prev, i as i32);
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
        debug_assert_eq!(find_max_length(&[0, 1]), 2);
        debug_assert_eq!(find_max_length(&[0, 1, 0]), 2);
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
