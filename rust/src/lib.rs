mod dsu;
mod helper;
mod trie;

use std::cmp::Reverse;

#[allow(unused_imports)]
use helper::*;

pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {
    if nums.len() <= k as usize {
        return nums;
    }
    let mut nis: Vec<_> = nums.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    let (res, _, _) = nis.select_nth_unstable_by_key(k as usize, |&(_i, v)| Reverse(v));
    res.sort_unstable_by_key(|&(i, _v)| i);
    res.iter().map(|&(_, v)| v).collect()
}

#[cfg(test)]
mod tests {

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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
    fn basics() {}

    #[test]
    fn test() {}
}
