mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_size_subarray(nums: &[i32], target: i32) -> i32 {
    use std::collections::HashMap;
    let k = i64::from(target);
    let n = nums.len();
    let sum: i64 = nums.iter().map(|&v| i64::from(v)).sum();
    let rem = k % sum;
    let len = (k / sum) as i32 * n as i32;
    if rem == 0 {
        return len;
    }
    let mut res = i32::MAX;
    let mut prefix = HashMap::new();
    let mut curr = 0;
    for (idx, &num) in nums.iter().enumerate() {
        curr += i64::from(num);
        if let Some(&left) = prefix.get(&(curr - rem)) {
            res = res.min((idx - left) as i32 + len);
        }
        let right_sum = sum - curr;
        if let Some(&left) = prefix.get(&(rem - right_sum)) {
            res = res.min((1 + left + n - idx - 1) as i32 + len);
        }
        prefix.insert(curr, idx);
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
        assert_eq!(min_size_subarray(&[1, 2, 3], 5), 2);
        assert_eq!(min_size_subarray(&[1, 1, 1, 2, 3], 4), 2);
        assert_eq!(min_size_subarray(&[2, 4, 6, 8], 3), -1);
    }

    #[test]
    fn test() {
        assert_eq!(min_size_subarray(&[2, 1, 5, 7, 7, 1, 6, 3], 39), 9);
    }
}
