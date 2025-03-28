mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix = nums.iter().fold(vec![0], |mut acc, &num| {
        acc.push(num + acc.last().unwrap_or(&0));
        acc
    });
    prefix.pop();
    let mut suffix = nums.iter().rev().fold(vec![0], |mut acc, &num| {
        acc.push(num + acc.last().unwrap_or(&0));
        acc
    });
    suffix.pop();
    suffix.reverse();
    prefix
        .into_iter()
        .zip(suffix)
        .map(|(a, b)| (a - b).abs())
        .collect()
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
