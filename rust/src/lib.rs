mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_length_after_removals(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut left = 0;
    let mut right = n / 2;
    let mut count = 0;
    while left < n / 2 && right < n {
        while nums.get(right).is_some_and(|&v| v == nums[left]) {
            right += 1;
        }
        if right >= n {
            break;
        }
        left += 1;
        right += 1;
        count += 2;
    }
    n as i32 - count
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
        assert_eq!(min_length_after_removals(&[1, 2, 3, 4]), 0);
    }

    #[test]
    fn test() {}
}
