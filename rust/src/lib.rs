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
    use std::collections::BTreeSet;
    let n = nums.len();
    let mut suf_mex = Vec::with_capacity(n);
    // All numbers not in `nums`
    let mut set: BTreeSet<_> = (0..=n as i32).collect();
    for &num in nums.iter().rev() {
        set.remove(&num); // Remove candidate
        suf_mex.push(*set.first().unwrap());
    }
    suf_mex.reverse();
    let mut mex = suf_mex[0];
    // These are required to reach `mex`
    // Put `mex` in for easier `first()` call
    let mut filler: BTreeSet<_> = (0..=mex).collect();
    let mut res = vec![];
    for (idx, &num) in nums.iter().enumerate() {
        filler.remove(&num);
        let v = *filler.first().unwrap();
        // Found all <=mex
        if v >= mex {
            res.push(v);
            filler.clear();
            if let Some(v) = suf_mex.get(1 + idx) {
                mex = *v;
                filler.extend(0..=mex);
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
