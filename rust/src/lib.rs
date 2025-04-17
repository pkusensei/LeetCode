mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
    nums.sort_unstable();
    let mut res = 0;
    for (i, &num) in nums.iter().enumerate() {
        let mut left = i;
        let mut right = nums.partition_point(|&v| v <= 2 * num);
        for bit in (0..=20).rev() {
            if left >= right {
                break;
            }
            let [nl, nr] = if num & (1 << bit) > 0 {
                // bit is set in [left]
                // confine search within range where it's unset
                [left, binary_search(&nums, left, right, 1 << bit)]
            } else {
                // Or search in all numbers that has it set
                [binary_search(&nums, left, right, 1 << bit), right]
            };
            if nl != nr {
                left = nl;
                right = nr;
            }
        }
        res = res.max(num ^ nums[left]);
    }
    res
}

// Find the min num with set bit in mask
fn binary_search(nums: &[i32], mut left: usize, mut right: usize, mask: i32) -> usize {
    if nums[left] & mask > 0 {
        return left;
    }
    if right.checked_sub(1).is_some_and(|i| nums[i] & mask == 0) {
        return right;
    }
    while left < right {
        let mid = left.midpoint(right);
        if nums[mid] & mask > 0 {
            right = mid;
        } else {
            left = 1 + mid;
        }
    }
    left
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
        assert_eq!(maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
        assert_eq!(maximum_strong_pair_xor(vec![10, 100]), 0);
        assert_eq!(maximum_strong_pair_xor(vec![500, 520, 2500, 3000]), 1020);
    }

    #[test]
    fn test() {}
}
