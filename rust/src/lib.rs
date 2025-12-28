mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(nums: &[i32]) -> i64 {
    let n = nums.len();
    let mut suf_min = nums.iter().rev().fold(vec![], |mut acc, &v| {
        acc.push(v.min(*acc.last().unwrap_or(&i32::MAX)));
        acc
    });
    suf_min.pop();
    suf_min.reverse();
    let mut prefix = 0;
    let mut max = i64::MIN;
    for (i, &num) in nums[..n - 1].iter().enumerate() {
        prefix += i64::from(num);
        let curr = prefix - i64::from(suf_min[i]);
        max = max.max(curr);
    }
    max
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
    fn basics() {
        assert_eq!(maximum_score(&[-7, -5, 3]), -2);
    }

    #[test]
    fn test() {}
}
