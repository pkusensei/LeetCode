mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_subarrays(nums: Vec<i32>) -> i32 {
    let bitand = nums.iter().fold(i32::MAX, |acc, &num| acc & num);
    if bitand > 0 {
        1
    } else {
        let mut curr = i32::MAX;
        let mut res = 0;
        for &num in nums.iter() {
            curr &= num;
            if curr == 0 {
                res += 1;
                curr = i32::MAX;
            }
        }
        res
    }
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
