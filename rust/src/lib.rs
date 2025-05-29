mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut lcp = vec![vec![0; 1 + n]; 1 + n];
    for i1 in (0..n).rev() {
        for i2 in i1..n {
            if nums[i1] == nums[i2] {
                lcp[i1][i2] = 1 + lcp[1 + i1][1 + i2];
            }
        }
    }
    let mut res = 0;
    for i1 in 1..n {
        for i2 in 1 + i1..n {
            // split between nums1 and nums2, i.e [0..i1) and [i1..i2)
            // This length must be >= i1, len(nums1)
            // Or
            // split between nums2 and nums3, i.e [i1..i2) and [i2..n)
            // length must be >= (i2-i1), len(nums2)
            if lcp[0][i1].min(i1).min(i2 - i1) >= i1
                || lcp[i1][i2].min(i2 - i1).min(n - i2) >= i2 - i1
            {
                res += 1;
            }
        }
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
        assert_eq!(beautiful_splits(&[1, 1, 2, 1]), 2);
        assert_eq!(beautiful_splits(&[1, 2, 3, 4]), 0);
    }

    #[test]
    fn test() {}
}
