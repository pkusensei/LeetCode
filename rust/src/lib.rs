mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_mex(nums: &[i32]) -> Vec<i32> {
    let n = nums.len();
    let max = *nums.iter().max().unwrap();
    let mut freq = nums
        .iter()
        .fold(vec![0; 2 + max as usize], |mut acc, &num| {
            acc[num as usize] += 1;
            acc
        });
    // We attempt to make this "mex"
    let mut target = 0;
    // Track which number still survives
    let mut mex_present = vec![false; 2 + max as usize];
    let mut res = vec![];
    for &num in nums.iter() {
        // Prior to consider `num`
        // Any present number in 0..target is not mex
        while mex_present[target] {
            target += 1;
        }
        // Found candidate
        if freq[target] == 0 {
            res.push(target as i32);
            // Remove all impact
            mex_present[..target].fill(false);
            target = 0;
        }
        // Add this number
        mex_present[num as usize] = true;
        // "Truncate" nums
        freq[num as usize] -= 1;
    }
    while mex_present[target] {
        target += 1;
    }
    if res.len() < n {
        res.push(target as i32);
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
        assert_eq!(maximum_mex(&[3, 1]), [0, 0]);
        assert_eq!(maximum_mex(&[0, 1, 0]), [2, 1]);
        assert_eq!(maximum_mex(&[1, 0, 2]), [3]);
    }

    #[test]
    fn test() {}
}
