mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_maximum_score(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut left = 0;
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        if num > nums[left] {
            res += (right - left) as i64 * i64::from(nums[left]);
            left = right;
        }
    }
    res += (n - 1).saturating_sub(left) as i64 * i64::from(nums[left]);
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
    fn basics() {}

    #[test]
    fn test() {}
}
