mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
    let mut freq = [0; 32];
    let mut res = 200_001;
    let mut left = 0;
    for (right, &num) in nums.iter().enumerate() {
        for (bit, v) in freq.iter_mut().enumerate() {
            if num & (1 << bit) > 0 {
                *v += 1;
            }
        }
        while left <= right
            && k <= freq.iter().enumerate().fold(
                0,
                |acc, (bit, &f)| {
                    if f > 0 { acc | (1 << bit) } else { acc }
                },
            )
        {
            res = res.min(1 + right - left);
            for (bit, v) in freq.iter_mut().enumerate() {
                if nums[left] & (1 << bit) > 0 {
                    *v -= 1;
                }
            }
            left += 1;
        }
    }
    if res < 200_001 { res as i32 } else { -1 }
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
