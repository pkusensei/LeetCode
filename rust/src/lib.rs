mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_be_increasing(nums: &[i32]) -> bool {
    nums.len() - find_lis(nums) <= 1
}

fn find_lis(nums: &[i32]) -> usize {
    let mut arr = vec![];
    for &num in nums {
        let i = arr.partition_point(|&v| v < num);
        if i == arr.len() {
            arr.push(num);
        } else {
            arr[i] = num;
        }
    }
    arr.len()
}

pub fn single_pass(nums: &[i32]) -> bool {
    let mut dropped = false;
    let mut prev = nums[0];
    for (idx, &num) in nums.iter().enumerate().skip(1) {
        if num <= prev {
            if dropped {
                return false;
            }
            dropped = true;
            if idx == 1 || num > nums[idx - 2] {
                prev = num; // drop [idx-1], else drop current num
            }
        } else {
            prev = num;
        }
    }
    true
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
        assert!(can_be_increasing(&[1, 2, 10, 5, 7]));
        assert!(!can_be_increasing(&[2, 3, 1, 2]));
        assert!(!can_be_increasing(&[1, 1, 1]));

        assert!(single_pass(&[1, 2, 10, 5, 7]));
        assert!(!single_pass(&[2, 3, 1, 2]));
        assert!(!single_pass(&[1, 1, 1]));
    }

    #[test]
    fn test() {}
}
