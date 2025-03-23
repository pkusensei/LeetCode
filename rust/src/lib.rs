mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_subarrays(nums: &[i32], k: i32) -> i32 {
    let Some(ki) = nums.iter().position(|&v| v == k) else {
        return 0;
    };
    let mut curr = 0;
    let mut res = 0;
    let mut map = std::collections::HashMap::new();
    for &num in &nums[ki..] {
        curr += (num - k).signum();
        res += i32::from(matches!(curr, 0 | 1));
        *map.entry(curr).or_insert(0) += 1;
    }
    curr = 0;
    for &num in nums[..ki].iter().rev() {
        curr += (num - k).signum();
        if let Some(v) = map.get(&-curr) {
            res += v;
        }
        if let Some(v) = map.get(&(1 - curr)) {
            res += v;
        }
    }
    res
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
        assert_eq!(count_subarrays(&[3, 2, 1, 4, 5], 4), 3);
        assert_eq!(count_subarrays(&[2, 3, 1], 3), 1);
    }

    #[test]
    fn test() {
        assert_eq!(
            count_subarrays(
                &[
                    5, 19, 11, 15, 13, 16, 4, 6, 2, 7, 10, 8, 18, 20, 1, 3, 17, 9, 12, 14
                ],
                6
            ),
            13
        );
    }
}
