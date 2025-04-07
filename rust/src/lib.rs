mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_array(nums: Vec<i32>, k: i32) -> bool {
    use itertools::Itertools;
    let mut nums = nums;
    let n = nums.len();
    let k = k as usize;
    for left in 0..=n - k {
        let curr = nums[left];
        if curr < 0 {
            return false;
        }
        if curr == 0 {
            continue;
        }
        for v in nums[left..left + k].iter_mut() {
            *v -= curr;
        }
    }
    for v in nums.iter().tail(k) {
        if *v != 0 {
            return false;
        }
    }
    true
}

pub fn sliding_window(nums: Vec<i32>, k: i32) -> bool {
    let mut nums = nums;
    let n = nums.len();
    let k = k as usize;
    let mut curr = 0;
    for idx in 0..n {
        if curr > nums[idx] {
            return false;
        }
        nums[idx] -= curr;
        curr += nums[idx];
        if idx >= k - 1 {
            curr -= nums[idx + 1 - k];
        }
    }
    curr == 0
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
        assert!(check_array(vec![2, 2, 3, 1, 1, 0], 3));
        assert!(!check_array(vec![1, 3, 1, 1], 2));

        assert!(sliding_window(vec![2, 2, 3, 1, 1, 0], 3));
        assert!(!sliding_window(vec![1, 3, 1, 1], 2));
    }

    #[test]
    fn test() {
        assert!(check_array(
            vec![
                60, 72, 87, 89, 63, 52, 64, 62, 31, 37, 57, 83, 98, 94, 92, 77, 94, 91, 87, 100,
                91, 91, 50, 26
            ],
            4
        ));
        assert!(sliding_window(
            vec![
                60, 72, 87, 89, 63, 52, 64, 62, 31, 37, 57, 83, 98, 94, 92, 77, 94, 91, 87, 100,
                91, 91, 50, 26
            ],
            4
        ));
    }
}
