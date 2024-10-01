mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn array_nesting(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = 0;
    let mut seen = HashSet::new();
    for &num in nums.iter() {
        res = res.max(dfs(nums, num, &mut seen));
        if res > n as i32 / 2 {
            return res;
        }
    }
    res
}

fn dfs(nums: &[i32], start: i32, seen: &mut HashSet<i32>) -> i32 {
    if !seen.insert(start) {
        return 0;
    }
    let mut num = nums[start as usize];
    let mut res = 1;
    while seen.insert(num) {
        num = nums[num as usize];
        res += 1
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(array_nesting(&[5, 4, 0, 3, 1, 6, 2]), 4);
        debug_assert_eq!(array_nesting(&[0, 1, 2]), 1);
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
