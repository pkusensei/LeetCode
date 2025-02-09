mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_frequency(nums: &mut [i32], k: i32) -> i32 {
    nums.sort_unstable();
    let mut res = 0;
    let mut left = 0;
    let mut sum = 0;
    for (right, &num) in nums.iter().enumerate() {
        sum += num as usize;
        while num as usize * (right + 1 - left) > sum + k as usize {
            sum -= nums[left] as usize;
            left += 1;
        }
        res = res.max(right + 1 - left);
    }
    res as _
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
        assert_eq!(max_frequency(&mut [1, 2, 4], 5), 3);
        assert_eq!(max_frequency(&mut [1, 4, 8, 13], 5), 2);
        assert_eq!(max_frequency(&mut [3, 9, 6], 2), 1);
    }

    #[test]
    fn test() {}
}
