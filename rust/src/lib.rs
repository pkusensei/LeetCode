mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_beauties(nums: &[i32]) -> i32 {
    let n = nums.len();
    let pre_max = nums.iter().fold(Vec::with_capacity(n), |mut acc, &num| {
        acc.push(num.max(acc.last().copied().unwrap_or(0)));
        acc
    });
    let mut suf_min = nums
        .iter()
        .rev()
        .fold(Vec::with_capacity(n), |mut acc, &num| {
            acc.push(num.min(acc.last().copied().unwrap_or(i32::MAX)));
            acc
        });
    suf_min.reverse();
    (1..n - 1)
        .map(|i| (i, nums[i]))
        .map(|(i, num)| {
            if pre_max[i - 1] < num && num < suf_min[1 + i] {
                2
            } else if nums[i - 1] < num && num < nums[1 + i] {
                1
            } else {
                0
            }
        })
        .sum()
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
        assert_eq!(sum_of_beauties(&[1, 2, 3]), 2);
        assert_eq!(sum_of_beauties(&[2, 4, 6, 4]), 1);
        assert_eq!(sum_of_beauties(&[3, 2, 1]), 0);
    }

    #[test]
    fn test() {}
}
