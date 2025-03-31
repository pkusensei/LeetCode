mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
    nums.sort_unstable();
    let mut left = 0;
    let mut right = 10i32.pow(9);
    while left < right {
        let mid = left + (right - left) / 2;
        if count(&nums, mid) >= p {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn count(nums: &[i32], mid: i32) -> i32 {
    let mut idx = 0;
    let mut res = 0;
    while let (Some(a), Some(b)) = (nums.get(idx), nums.get(1 + idx)) {
        if b - a <= mid {
            res += 1;
            idx += 2
        } else {
            idx += 1;
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
        assert_eq!(minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
        assert_eq!(minimize_max(vec![4, 2, 1, 2], 1), 0);
    }

    #[test]
    fn test() {}
}
