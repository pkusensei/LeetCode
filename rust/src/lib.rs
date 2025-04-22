mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_array_length(nums: Vec<i32>) -> i32 {
    let mut min = i32::MAX;
    let mut count = 0;
    for &num in &nums {
        if num < min {
            min = num;
            count = 1;
        } else if num == min {
            count += 1;
        }
    }
    if count == 1 {
        return 1;
    }
    if nums.iter().any(|&v| v % min > 0) {
        return 1;
    }
    (1 + count) / 2
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
