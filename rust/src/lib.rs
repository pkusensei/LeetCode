mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_replacement(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut last = nums[n - 1];
    let mut res = 0;
    for &num in nums.iter().rev().skip(1) {
        if num <= last {
            last = num;
        } else {
            let div = num / last + i32::from(num % last > 0);
            last = num / div;
            res += i64::from(div - 1);
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
        assert_eq!(minimum_replacement(&[3, 9, 3]), 2);
    }

    #[test]
    fn test() {
        assert_eq!(minimum_replacement(&[12, 9, 7, 6, 17, 19, 21]), 6);
        assert_eq!(minimum_replacement(&[2, 10, 20, 19, 1]), 47);
    }
}
