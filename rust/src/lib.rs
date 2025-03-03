mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let n = nums.len();
    let mut res = vec![0; n];
    // ptrs into nums array
    let mut left = 0;
    let mut right = n - 1;
    // ptrs into res array
    let mut small = 0;
    let mut big = n - 1;
    while left < n {
        if nums[left] < pivot {
            res[small] = nums[left];
            small += 1;
        }
        if nums[right] > pivot {
            res[big] = nums[right];
            big -= 1;
        }
        left += 1;
        right -= 1;
    }
    while small <= big {
        res[small] = pivot;
        small += 1;
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
    fn basics() {}

    #[test]
    fn test() {}
}
