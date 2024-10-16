mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn delete_and_earn(nums: &mut [i32]) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    nums.sort_unstable();
    let (min, max) = (nums[0], *nums.last().unwrap());
    let map: HashMap<_, _> = nums
        .chunk_by(|a, b| a == b)
        .map(|ch| (ch[0], ch.iter().sum()))
        .collect();

    // and house robber
    let mut prev = 0;
    let mut res = 0;
    for num in min..=max {
        let val = map.get(&num).copied().unwrap_or(0);
        let temp = res;
        res = res.max(prev + val);
        prev = temp;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(delete_and_earn(&mut [3, 4, 2]), 6);
        debug_assert_eq!(delete_and_earn(&mut [2, 2, 3, 3, 3, 4]), 9);
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
