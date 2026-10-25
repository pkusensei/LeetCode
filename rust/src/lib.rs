mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i64 {
    let sum: i64 = nums.iter().map(|&v| v as i64).sum();
    let d: i64 = nums
        .iter()
        .map(|&v| delta(v))
        .sorted_unstable_by(|a, b| b.cmp(a))
        .take(k as usize)
        .sum();
    sum + d
}

fn delta(num: i32) -> i64 {
    let mut s = num.to_string().into_bytes();
    s.sort_unstable_by(|a, b| b.cmp(a));
    let big: i64 = String::from_utf8(s).unwrap().parse().unwrap();
    big - i64::from(num)
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
