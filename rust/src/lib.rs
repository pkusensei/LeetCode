mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_indices(nums: &[i32], index_difference: i32, value_difference: i32) -> Vec<i32> {
    let idiff = index_difference as usize;
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let [mut mini, mut maxi] = [nums.len(); 2];
    for (right, &num) in nums.iter().enumerate().skip(idiff) {
        let left = right - idiff;
        if (nums[left]) < min {
            min = nums[left];
            mini = left;
        };
        if (nums[left]) > max {
            max = nums[left];
            maxi = left;
        }
        if (min - num).abs() >= value_difference {
            return vec![mini as i32, right as i32];
        }
        if (max - num).abs() >= value_difference {
            return vec![maxi as i32, right as i32];
        }
    }
    vec![-1, -1]
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
        assert_eq!(find_indices(&[5, 1, 4, 1], 2, 4), [0, 3]);
        assert_eq!(find_indices(&[2, 1], 0, 0), [0, 0]);
        assert_eq!(find_indices(&[1, 2, 3], 2, 4), [-1, -1]);
    }

    #[test]
    fn test() {}
}
