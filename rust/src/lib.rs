mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn incremovable_subarray_count(nums: &[i32]) -> i32 {
    let mut res = 0;
    let n = nums.len();
    for left in 0..n {
        'outer: for right in left..n {
            let mut prev = -1;
            for (idx, &num) in nums.iter().enumerate() {
                if idx < left || idx > right {
                    if prev >= num {
                        continue 'outer;
                    }
                    prev = num;
                }
            }
            res += 1;
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
        assert_eq!(incremovable_subarray_count(&[1, 2, 3, 4]), 10);
        assert_eq!(incremovable_subarray_count(&[6, 5, 7, 8]), 7);
        assert_eq!(incremovable_subarray_count(&[8, 7, 6, 6]), 3);
    }

    #[test]
    fn test() {}
}
