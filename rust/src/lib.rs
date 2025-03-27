mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut res = 0;
    let mut left = 0;
    let mut right = n - 1;
    while left <= right && left < n {
        if left == right {
            res += i64::from(nums[left])
        } else {
            let v1 = i64::from(nums[left]);
            let v2 = i64::from(nums[right]);
            res += v1 * 10i64.pow(1 + v2.ilog10()) + v2;
        }
        left += 1;
        right -= 1;
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
