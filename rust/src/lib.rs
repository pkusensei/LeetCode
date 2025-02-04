mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_size(nums: &[i32], max_operations: i32) -> i32 {
    let n = nums.len() as i32;
    let mut left = 1;
    let mut right = *nums.iter().max().unwrap();
    while left < right {
        let mid = left + (right - left) / 2;
        if count(nums, mid) <= n + max_operations {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left
}

fn count(nums: &[i32], mid: i32) -> i32 {
    nums.iter().map(|v| v / mid + i32::from(v % mid > 0)).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(minimum_size(&[9], 2), 3);
        assert_eq!(minimum_size(&[2, 4, 8, 2], 4), 2);
    }

    #[test]
    fn test() {}
}
