mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_product(nums: &[i32], m: i32) -> i64 {
    let m = m as usize;
    let mut left_max = i32::MIN;
    let mut left_min = i32::MAX;
    let mut res = i64::MIN;
    for (i, &num) in nums.iter().enumerate() {
        if i + 1 >= m {
            let left = nums[i + 1 - m];
            left_max = left_max.max(left);
            left_min = left_min.min(left);
            res = res
                .max(i64::from(num) * i64::from(left_max))
                .max(i64::from(num) * i64::from(left_min));
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
    fn basics() {}

    #[test]
    fn test() {}
}
