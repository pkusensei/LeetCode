mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_subarrays(nums: &[i32], k: i32) -> i64 {
    let max = nums.iter().copied().max().unwrap();
    let mut count = 0;
    let mut left = 0;
    let mut res = 0;
    for (right, &num) in nums.iter().enumerate() {
        count += i32::from(num == max);
        while count >= k {
            count -= i32::from(nums[left] == max);
            left += 1;
        }
        res += right + 1 - left;
    }
    let n = nums.len();
    (n * (1 + n) / 2 - res) as i64
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
