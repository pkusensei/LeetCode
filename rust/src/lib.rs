mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n == 1 {
        return 1;
    }
    let min = *nums.iter().min().unwrap();
    let max = *nums.iter().max().unwrap();
    let mini = nums.iter().position(|&v| v == min).unwrap();
    let maxi = nums.iter().position(|&v| v == max).unwrap();
    let left = mini.min(maxi);
    let right = mini.max(maxi);
    (1 + right).min(n - left).min(n - (right - left - 1)) as i32
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
