mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn longest_monotonic_subarray(nums: &[i32]) -> i32 {
    let mut res = 1;
    let [mut curr_inc, mut curr_dec] = [1, 1];
    let mut prev = nums[0];
    for &num in nums.iter() {
        match prev.cmp(&num) {
            std::cmp::Ordering::Less => {
                curr_inc += 1;
                curr_dec = 1;
            }
            std::cmp::Ordering::Equal => {
                curr_inc = 1;
                curr_dec = 1;
            }
            std::cmp::Ordering::Greater => {
                curr_inc = 1;
                curr_dec += 1
            }
        }
        prev = num;
        res = res.max(curr_inc).max(curr_dec);
    }
    res
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
        assert_eq!(longest_monotonic_subarray(&[1, 4, 3, 3, 2]), 2);
        assert_eq!(longest_monotonic_subarray(&[3, 3, 3, 3]), 1);
        assert_eq!(longest_monotonic_subarray(&[3, 2, 1]), 3);
    }

    #[test]
    fn test() {}
}
