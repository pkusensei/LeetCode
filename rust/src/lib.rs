mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn peak_index_in_mountain_array(arr: &[i32]) -> i32 {
    let n = arr.len();
    let (mut left, mut right) = (0, n - 1);
    while left < right {
        let mid = left + (right - left) / 2;
        if (arr[mid - 1]..arr[mid + 1]).contains(&arr[mid]) {
            left = mid
        } else if (arr[mid + 1]..arr[mid - 1]).contains(&arr[mid]) {
            right = mid
        } else {
            return mid as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(peak_index_in_mountain_array(&[0, 1, 0]), 1);
        debug_assert_eq!(peak_index_in_mountain_array(&[0, 2, 1, 0]), 1);
        debug_assert_eq!(peak_index_in_mountain_array(&[0, 2, 1, 0]), 1);
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
