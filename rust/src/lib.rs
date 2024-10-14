mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_length(nums1: &[i32], nums2: &[i32]) -> i32 {
    let (n1, n2) = (nums1.len(), nums2.len());
    let mut dp = vec![vec![0; 1 + n2]; 1 + n1];
    for i1 in 1..1 + n1 {
        for i2 in 1..1 + n2 {
            if nums1[i1 - 1] == nums2[i2 - 1] {
                dp[i1][i2] = 1 + dp[i1 - 1][i2 - 1]
            }
        }
    }
    dp.into_iter()
        .flat_map(|v| v.into_iter())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_length(&[1, 2, 3, 2, 1], &[3, 2, 1, 4, 7]), 3);
        debug_assert_eq!(find_length(&[0, 0, 0, 0, 0], &[0, 0, 0, 0, 0]), 5);
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
