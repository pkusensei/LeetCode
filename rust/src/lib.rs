mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_subarray(nums: &[i32], p: i32) -> i32 {
    let sum: i32 = nums.iter().fold(0, |acc, &n| (acc + n) % p);
    let target = sum % p;
    if target == 0 {
        return 0;
    };
    let mut map = std::collections::HashMap::from([(0, -1)]);
    let (mut curr, mut res) = (0, nums.len() as i32);
    for (i, &num) in nums.iter().enumerate() {
        curr = (curr + num) % p;
        let v = (curr - target).rem_euclid(p);
        if let Some(j) = map.get(&v) {
            res = res.min(i as i32 - j);
        }
        map.insert(curr, i as i32);
    }
    if res == nums.len() as i32 {
        -1
    } else {
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_subarray(&[3, 1, 4, 2], 6), 1);
        debug_assert_eq!(min_subarray(&[6, 3, 5, 2], 9), 2);
        debug_assert_eq!(min_subarray(&[1, 2, 3], 3), 0);
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
