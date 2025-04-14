mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_sum(nums: &[i32]) -> i32 {
    let n = nums.len();
    let [mut prefix, mut suffix] = [Vec::with_capacity(n), Vec::with_capacity(n)];
    for &num in nums.iter() {
        prefix.push(num.min(*prefix.last().unwrap_or(&i32::MAX)));
    }
    for &num in nums.iter().rev() {
        suffix.push(num.min(*suffix.last().unwrap_or(&i32::MAX)));
    }
    suffix.reverse();
    let mut res = i32::MAX;
    for i in 1..n - 1 {
        if prefix[i - 1] < nums[i] && nums[i] > suffix[1 + i] {
            res = res.min(prefix[i - 1] + nums[i] + suffix[1 + i]);
        }
    }
    if res == i32::MAX { -1 } else { res }
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
        assert_eq!(minimum_sum(&[8, 6, 1, 5, 3]), 9);
        assert_eq!(minimum_sum(&[5, 4, 8, 7, 10, 2]), 13);
        assert_eq!(minimum_sum(&[6, 5, 4, 3, 4, 5]), -1);
    }

    #[test]
    fn test() {}
}
