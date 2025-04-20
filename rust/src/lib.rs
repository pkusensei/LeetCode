mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_frequency_score(mut nums: Vec<i32>, k: i64) -> i32 {
    nums.sort_unstable();
    let prefix = nums.iter().fold(vec![], |mut acc, &num| {
        acc.push(i64::from(num) + acc.last().unwrap_or(&0));
        acc
    });
    let mut left = 1;
    let mut right = nums.len();
    while left < right {
        let mid = left.midpoint(right + 1);
        if check(&nums, k, &prefix, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    left as i32
}

fn check(nums: &[i32], k: i64, prefix: &[i64], size: usize) -> bool {
    let n = nums.len();
    for left in 0..=n - size {
        let right = left + size - 1;
        let mid = left.midpoint(right);
        let sum_left = prefix[mid] - if left > 0 { prefix[left - 1] } else { 0 };
        let sum_right = prefix[right] - prefix[mid];
        let mid_val = i64::from(nums[mid]);
        if (mid + 1 - left) as i64 * mid_val - sum_left + sum_right - (right - mid) as i64 * mid_val
            <= k
        {
            return true;
        }
    }
    false
}

pub fn sliding_window(mut nums: Vec<i32>, k: i64) -> i32 {
    nums.sort_unstable();
    let mut left: usize = 0;
    let mut res = 1;
    let mut med = 0;
    let mut cost = 0;
    for (right, &num) in nums.iter().enumerate().skip(1) {
        cost += i64::from(num - nums[med]);
        med = left.midpoint(right + 1);
        while cost > k {
            cost -= i64::from(nums[med] - nums[left]);
            left += 1;
            med = left.midpoint(1 + right);
        }
        res = res.max(right + 1 - left)
    }
    res as i32
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
        assert_eq!(max_frequency_score(vec![1, 2, 6, 4], 3), 3);
        assert_eq!(max_frequency_score(vec![1, 4, 4, 2, 4], 0), 3);

        assert_eq!(sliding_window(vec![1, 2, 6, 4], 3), 3);
        assert_eq!(sliding_window(vec![1, 4, 4, 2, 4], 0), 3);
    }

    #[test]
    fn test() {}
}
