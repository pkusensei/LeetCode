mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_absolute_sum(nums: &[i32]) -> i32 {
    let mut res = i32::MIN;
    let [mut curr_max, mut curr_min] = [0, 0];
    for &num in nums.iter() {
        curr_max = num.max(curr_max + num);
        curr_min = num.min(curr_min + num);
        res = res.max(curr_max.abs()).max(curr_min.abs())
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
        assert_eq!(max_absolute_sum(&[1, -3, 2, 3, -4]), 5);
        assert_eq!(max_absolute_sum(&[2, -5, 1, -4, 3, -2]), 8);
    }

    #[test]
    fn test() {}
}
