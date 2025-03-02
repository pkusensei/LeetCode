mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_elements(nums: Vec<i32>) -> i32 {
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    if min == max {
        return 0;
    }
    let n = nums.len();
    let min_count = nums.iter().filter(|&&v| v == min).count();
    let max_count = nums.iter().filter(|&&v| v == max).count();
    (n - min_count - max_count) as _
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
