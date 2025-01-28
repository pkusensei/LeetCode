mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_deviation(nums: &[i32]) -> i32 {
    let mut heap = std::collections::BinaryHeap::new();
    let mut min = i32::MAX;
    // all odd=>even and keep track of mins
    for &num in nums.iter() {
        let v = if num & 1 == 1 { 2 * num } else { num };
        min = min.min(v);
        heap.push(v);
    }
    let mut res = i32::MAX;
    while let Some(num) = heap.pop() {
        // track current delta
        res = res.min(num - min);
        if num & 1 == 0 {
            // update min if possible
            min = min.min(num / 2);
            heap.push(num / 2);
        } else {
            break;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(minimum_deviation(&[1, 2, 3, 4]), 1);
        assert_eq!(minimum_deviation(&[4, 1, 5, 20, 3]), 3);
        assert_eq!(minimum_deviation(&[2, 10, 8]), 3);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
