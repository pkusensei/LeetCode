mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_disappeared_numbers(mut nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut res = vec![];
    let mut prev = lower;
    for &num in nums.iter() {
        if prev <= num - 1 && prev <= upper {
            res.push(vec![prev, (num - 1).min(upper)]);
        }
        prev = (1 + num).max(prev);
    }
    if prev <= upper {
        res.push(vec![prev, upper]);
    }
    res
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
    fn basics() {
        assert_eq!(
            find_disappeared_numbers(vec![3, 9, 7], 1, 12),
            [[1, 2], [4, 6], [8, 8], [10, 12]]
        );
    }

    #[test]
    fn test() {}
}
