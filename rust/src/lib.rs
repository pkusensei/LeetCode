mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_index(nums: &[i32]) -> i32 {
    let n = nums.len();
    // let (major, mcount) = with_hashmap(nums);
    let (major, mcount) = with_boyer_moore(nums);
    let mut left = 0;
    let mut right = mcount;
    for (idx, &num) in (0..).zip(nums.iter()) {
        left += usize::from(num == major);
        right -= usize::from(num == major);
        if left > (1 + idx) / 2 && right > (n - 1 - idx) / 2 {
            return idx as i32;
        }
    }
    -1
}

fn with_hashmap(nums: &[i32]) -> (i32, usize) {
    use itertools::Itertools;
    let n = nums.len();
    let count = nums.iter().copied().counts();
    let (&major, &mcount) = count.iter().find(|(_, v)| **v > n / 2).unwrap();
    (major, mcount)
}

fn with_boyer_moore(nums: &[i32]) -> (i32, usize) {
    let mut count = 0;
    let mut major = nums[0];
    for &num in nums {
        if num == major {
            count += 1
        } else {
            count -= 1
        }
        if count == 0 {
            major = num;
            count = 1;
        }
    }
    (major, nums.iter().filter(|&&v| v == major).count())
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
        assert_eq!(minimum_index(&[1, 2, 2, 2]), 2);
        assert_eq!(minimum_index(&[2, 1, 3, 1, 1, 1, 7, 1, 2, 1]), 4);
        assert_eq!(minimum_index(&[3, 3, 3, 3, 7, 2, 2]), -1);
    }

    #[test]
    fn test() {}
}
