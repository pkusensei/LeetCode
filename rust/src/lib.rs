mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn four_sum_count(nums1: &[i32], nums2: &[i32], nums3: &[i32], nums4: &[i32]) -> i32 {
    let mut map = std::collections::HashMap::<i32, [i32; 2]>::new();
    for n1 in nums1.iter() {
        for n2 in nums2.iter() {
            map.entry(n1 + n2).or_default()[0] += 1;
        }
    }
    for n3 in nums3.iter() {
        for n4 in nums4.iter() {
            if let Some(v) = map.get_mut(&-(n3 + n4)) {
                v[1] += 1
            }
        }
    }
    map.into_values().map(|[x, y]| x * y).sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(four_sum_count(&[1, 2], &[-2, -1], &[-1, 2], &[0, 2]), 2);
        debug_assert_eq!(four_sum_count(&[0], &[0], &[0], &[0]), 1);
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
