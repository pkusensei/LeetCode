mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut res = i32::MAX;
    for right in 0..n {
        res = res.min((nums[right] - k).abs());
        let Some(mut left) = right.checked_sub(1) else {
            continue;
        };
        // Add bits on right to all left numbers
        // Once [left]|[right]==[left]
        // This [right] does not add any more to any accumulated [left]
        while nums[left] | nums[right] != nums[left] {
            nums[left] |= nums[right];
            res = res.min((nums[left] - k).abs());
            if left == 0 {
                break;
            }
            left -= 1;
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
        assert_eq!(minimum_difference(vec![1, 2, 4, 5], 3), 0);
        assert_eq!(minimum_difference(vec![1, 3, 1, 3], 2), 1);
        assert_eq!(minimum_difference(vec![1], 10), 9);
    }

    #[test]
    fn test() {}
}
