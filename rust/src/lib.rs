mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn apply_operations(mut nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n - 1 {
        if nums[i] == nums[1 + i] {
            nums[i] *= 2;
            nums[1 + i] = 0;
        }
    }
    let [mut left, mut right] = [0, 1];
    while right < n {
        while nums.get(right).is_some_and(|&v| v == 0) {
            right += 1;
        }
        if left < right && right < n && nums[left] == 0 {
            nums.swap(left, right);
            right += 1;
        }
        left += 1;
        right = right.max(1 + left);
    }
    nums
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
        assert_eq!(apply_operations(vec![1, 2, 2, 1, 1, 0]), [1, 4, 2, 0, 0, 0]);
    }

    #[test]
    fn test() {}
}
