mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// negate = sum + 2(-x)
// sum+(-2x).rem_euclid(k) = prev_sum%k
pub fn longest_subarray(nums: &[i32], k: i32) -> i32 {
    let n = nums.len();
    let k = i64::from(k);
    let mut prev_sum = vec![n as i32; k as usize];
    prev_sum[0] = -1;
    let mut prev_nums = vec![-1; k as usize];
    let mut sum = 0;
    let mut res = 0;
    for (idx, &num) in nums.iter().enumerate() {
        sum = (sum + i64::from(num)).rem_euclid(k);
        prev_nums[(-2 * i64::from(num)).rem_euclid(k) as usize] = idx as i32;
        if prev_sum[sum as usize] < n as i32 {
            res = res.max(idx as i32 - prev_sum[sum as usize]);
        }
        for (rem, &v) in prev_nums.iter().enumerate() {
            if v == -1 {
                continue; // !!! important prune !!!
            }
            let prev = prev_sum[((sum + rem as i64) % k) as usize];
            if prev < v {
                res = res.max(idx as i32 - prev)
            }
        }
        if prev_sum[sum as usize] == n as i32 {
            prev_sum[sum as usize] = idx as i32;
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
    fn test() {
        assert_eq!(longest_subarray(&[9, -12], 4), 1);
    }
}
