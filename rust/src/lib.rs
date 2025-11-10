mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_ideal_permutation(nums: &[i32]) -> bool {
    let n = nums.len();
    if n <= 2 {
        return true;
    }
    let mut max = 0;
    for i in 0..n - 2 {
        max = max.max(nums[i]);
        if max > nums[2 + i] {
            return false; // All global inversions must be local
        }
    }
    true
}

pub fn constrain_dislocation(nums: &[i32]) -> bool {
    nums.iter()
        .enumerate()
        .all(|(i, &v)| i.abs_diff(v as usize) <= 1)
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
        assert!(is_ideal_permutation(&[1, 0, 2]));
        assert!(!is_ideal_permutation(&[1, 2, 0]));

        assert!(constrain_dislocation(&[1, 0, 2]));
        assert!(!constrain_dislocation(&[1, 2, 0]));
    }

    #[test]
    fn test() {}
}
