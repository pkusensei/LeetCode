mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn pivot_index(nums: &[i32]) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut left = 0;
    for (idx, &num) in nums.iter().enumerate() {
        if sum - num - left == left {
            return idx as _;
        }
        left += num
    }
    -1
}

fn with_prefix(nums: &[i32]) -> i32 {
    let prefix = nums.iter().scan(0, |sum, num| {
        *sum += num;
        Some(*sum)
    });
    let suffix = nums.iter().rev().scan(0, |sum, num| {
        *sum += num;
        Some(*sum)
    });
    let (prefix, suffix) = (prefix.collect::<Vec<_>>(), suffix.collect::<Vec<_>>());
    // dbg!(prefix);
    // dbg!(suffix);
    prefix
        .into_iter()
        .zip(suffix.into_iter().rev())
        .position(|(a, b)| a == b)
        .map(|i| i as i32)
        .unwrap_or(-1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(with_prefix(&[1, 7, 3, 6, 5, 6]), 3);
        debug_assert_eq!(with_prefix(&[1, 2, 3]), -1);
        debug_assert_eq!(with_prefix(&[2, 1, -1]), 0);
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
