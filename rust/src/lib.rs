mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_digit_differences(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut res = 0;
    for pow in 0.. {
        if nums[0] < 10_i32.pow(pow) {
            break;
        }
        let mut freq = [0; 10];
        for &num in nums.iter() {
            let d = (num / 10i32.pow(pow)) % 10;
            freq[d as usize] += 1;
        }
        res += freq.into_iter().map(|v| v * (n - v)).sum::<usize>() / 2;
    }
    res as i64
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
        assert_eq!(sum_digit_differences(&[13, 23, 12]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(sum_digit_differences(&[50, 28, 48]), 5);
    }
}
