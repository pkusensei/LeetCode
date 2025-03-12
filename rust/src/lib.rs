mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn partition_array(mut nums: Vec<i32>, k: i32) -> i32 {
    nums.sort_unstable();
    let mut left = 0;
    let mut res = 1;
    for (right, &num) in nums.iter().enumerate() {
        if num - nums[left] > k {
            res += 1;
            left = right
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
    fn basics() {}

    #[test]
    fn test() {}
}
