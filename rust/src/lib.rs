mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarray(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut map = HashMap::new();
    let mut res = 2.min(n);
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        while f(&map, num) {
            let v = map.entry(nums[left]).or_insert(0);
            *v -= 1;
            if *v == 0 {
                map.remove(&nums[left]);
            }
            left += 1;
        }
        *map.entry(num).or_insert(0) += 1;
        res = res.max(1 + right - left);
    }
    res as i32

    // let mut left = 2;
    // let mut right = n;
    // while left < right {
    //     let mid = left + (right - left) / 2;
    // }
    // left as i32
}

fn f(map: &HashMap<i32, i32>, num: i32) -> bool {
    for (k, v) in map.iter() {
        if map.contains_key(&(k + num)) {
            return true;
        }
        let d = k.abs_diff(num) as i32;
        if d == *k {
            if *v > 1 {
                return true;
            }
        } else if map.contains_key(&d) {
            return true;
        }
    }
    false
}

// fn f(nums: &[i32], mid: usize) -> bool {
//     let mut map = HashMap::new();
//     for (idx, &num) in nums.iter().enumerate() {
//         if idx >= mid - 1 {}
//         if idx >= mid {}
//     }
//     true
// }

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {
        assert_eq!(max_subarray(&[3, 4, 5, 6]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(max_subarray(&[19, 28, 30, 19, 12, 5, 11, 22, 17, 1, 21]), 6);
    }
}
