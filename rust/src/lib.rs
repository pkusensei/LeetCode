mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn matrix_sum(mut nums: Vec<Vec<i32>>) -> i32 {
    let cols = nums[0].len();
    for v in nums.iter_mut() {
        v.sort_unstable_by(|a, b| b.cmp(a));
    }
    (0..cols)
        .map(|c| nums.iter().map(|r| r[c]).max().unwrap_or(0))
        .sum()
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
