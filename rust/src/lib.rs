mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut sorted = nums.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut res = i32::MAX;
    for (left, &min) in sorted.iter().enumerate() {
        let max = n as i32 - 1 + min;
        let right = sorted.partition_point(|&v| v <= max);
        let len = right - left;
        res = res.min((n - len) as i32);
    }
    res
}

pub fn sliding_window(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut sorted = nums.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    let mut res = i32::MAX;
    let mut right = 0;
    for (left, &min) in sorted.iter().enumerate() {
        while sorted.get(right).is_some_and(|&v| v < min + n as i32) {
            right += 1;
        }
        let len = right - left;
        res = res.min((n - len) as i32);
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
        assert_eq!(min_operations(&[4, 2, 5, 3]), 0);
        assert_eq!(min_operations(&[1, 2, 3, 5, 6]), 1);
        assert_eq!(min_operations(&[1, 10, 100, 1000]), 3);

        assert_eq!(sliding_window(&[4, 2, 5, 3]), 0);
        assert_eq!(sliding_window(&[1, 2, 3, 5, 6]), 1);
        assert_eq!(sliding_window(&[1, 10, 100, 1000]), 3);
    }

    #[test]
    fn test() {}
}
