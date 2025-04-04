mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn semi_ordered_permutation(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let (mut min, mut mini) = (i32::MAX, 0);
    let (mut max, mut maxi) = (0, 0);
    for (idx, &num) in nums.iter().enumerate() {
        if num < min {
            min = num;
            mini = idx;
        }
        if num > max {
            max = num;
            maxi = idx;
        }
    }
    if mini < maxi {
        (mini + n - maxi - 1) as i32
    } else {
        (mini + n - maxi - 1 - 1) as i32
    }
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
