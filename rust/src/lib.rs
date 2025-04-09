mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_absolute_difference(nums: &[i32], x: i32) -> i32 {
        use std::collections::BTreeSet;
        let x = x as usize;
        let mut set = BTreeSet::new();
        let mut res = i32::MAX;
        for (idx, &num) in nums.iter().enumerate().skip(x) {
            set.insert(nums[idx - x]);
            if let Some(v) = set.range(num..).next() {
                res = res.min((num - v).abs());
            }
            if let Some(v) = set.range(..num).next_back() {
                res = res.min((num - v).abs());
            }
        }
        res
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
    fn basics() {
        assert_eq!(min_absolute_difference(&[4, 3, 2, 4], 2), 0);
        assert_eq!(min_absolute_difference(&[5, 3, 2, 10, 15], 1), 1);
        assert_eq!(min_absolute_difference(&[1, 2, 3, 4], 3), 3);
    }

    #[test]
    fn test() {}
}
