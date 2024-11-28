mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_in_mountain_array(target: i32, arr: &MountainArray) -> i32 {
    let n = arr.length();
    let (mut left, mut right) = (0, n - 1);
    while left < right {
        let mid = left + (right - left) / 2;
        if arr.get(mid) < arr.get(mid + 1) {
            left = 1 + mid;
        } else {
            right = mid;
        }
    }
    search(arr, target, 0, left, |a, b| a < b)
        .or(search(arr, target, left, n - 1, |a, b| a > b))
        .unwrap_or(-1)
}

fn search(
    arr: &MountainArray,
    target: i32,
    mut left: i32,
    mut right: i32,
    f: fn(i32, i32) -> bool,
) -> Option<i32> {
    while left < right {
        let mid = left + (right - left) / 2;
        let curr = arr.get(mid);
        if curr == target {
            return Some(mid);
        } else if f(curr, target) {
            left = 1 + mid;
        } else {
            right = mid - 1;
        }
    }
    None
}

struct MountainArray;
impl MountainArray {
    fn get(&self, index: i32) -> i32 {
        42
    }
    fn length(&self) -> i32 {
        42
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
