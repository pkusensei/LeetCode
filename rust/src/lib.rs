mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn score_difference(nums: &[i32]) -> i32 {
    let mut scores = [0; 2];
    let mut active = 0;
    for (i, &num) in nums.iter().enumerate() {
        if num & 1 == 1 {
            active ^= 1;
        }
        if (1 + i) % 6 == 0 {
            active ^= 1;
        }
        scores[active] += num;
    }
    scores[0] - scores[1]
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
        assert_eq!(score_difference(&[2, 4, 2, 1, 2, 1]), 4);
    }

    #[test]
    fn test() {}
}
