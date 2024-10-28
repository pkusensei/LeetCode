mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_square_streak(nums: &mut [i32]) -> i32 {
    nums.sort_unstable();
    let mut seen = std::collections::HashSet::new();
    let mut res = -1;
    for &(mut num) in nums.iter() {
        if !seen.insert(num) {
            continue;
        }
        let mut count = 1;
        while num
            .checked_mul(num)
            .is_some_and(|v| nums.binary_search(&v).is_ok())
        {
            num *= num;
            seen.insert(num);
            count += 1;
        }
        if count > 1 {
            res = res.max(count);
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
        debug_assert_eq!(longest_square_streak(&mut [4, 3, 6, 16, 8, 2]), 3);
        debug_assert_eq!(longest_square_streak(&mut [2, 3, 5, 6, 7]), -1);
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
