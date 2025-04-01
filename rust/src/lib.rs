mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_or(nums: &[i32], k: i32) -> i64 {
    let prefix = nums.iter().fold(vec![], |mut acc, &num| {
        acc.push(i64::from(num) | acc.last().unwrap_or(&0));
        acc
    });
    let mut suffix = nums.iter().rev().fold(vec![], |mut acc, &num| {
        acc.push(i64::from(num) | acc.last().unwrap_or(&0));
        acc
    });
    suffix.reverse();
    let mut res = 0;
    for (i, &num) in nums.iter().enumerate() {
        let left = if i > 0 { prefix[i - 1] } else { 0 };
        let right = *suffix.get(1 + i).unwrap_or(&0);
        let curr = (i64::from(num) << k) | left | right;
        res = res.max(curr);
    }
    res
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
        assert_eq!(maximum_or(&[12, 9], 1), 30);
        assert_eq!(maximum_or(&[8, 1, 2], 2), 35);
    }

    #[test]
    fn test() {}
}
