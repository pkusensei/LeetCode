mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_mountain(arr: &[i32]) -> i32 {
    let n = arr.len();
    let mut idx = 0;
    let mut res = 0;
    while idx < n {
        let start = idx;
        while arr.get(1 + idx).is_some_and(|&v| arr[idx] < v) {
            idx += 1; // try find peak
        }
        if idx == start {
            idx += 1; // not valid peak
            continue;
        }
        let peak = idx;
        while arr.get(1 + idx).is_some_and(|&v| arr[idx] > v) {
            idx += 1
        }
        if peak == idx {
            idx += 1; // possible plateau
            continue;
        }
        res = res.max(idx - start + 1);
    }
    res as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_mountain(&[2, 1, 4, 7, 3, 2, 5]), 5);
        debug_assert_eq!(longest_mountain(&[2, 2, 2]), 0);
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
