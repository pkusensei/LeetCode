mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn min_patches(nums: &[i32], n: i32) -> i32 {
    // array [1..a] covers ints [1..m] where m == sum([1..a])
    // plug in m+1
    // so that [1..a, m+1] covers [1..m+m+1]
    let mut i = 0;
    let mut count = 0;
    let mut curr = 0;
    while curr < i64::from(n) as i64 {
        if i < nums.len() && i64::from(nums[i]) <= curr + 1 {
            // trying to build up to n
            curr += i64::from(nums[i]);
            i += 1
        } else {
            // but there's a gap
            count += 1;
            // plug in curr+1
            // so that now it covers [1..2*curr+1]
            curr = 2 * curr + 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_patches(&[1, 3], 6), 1);
        debug_assert_eq!(min_patches(&[1, 5, 10], 20), 2);
        debug_assert_eq!(min_patches(&[1, 2, 2], 5), 0);
    }

    #[test]
    fn test() {}

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
