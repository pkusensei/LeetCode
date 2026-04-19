mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let mut pref = nums.to_vec();
    for i in 1..n {
        pref[i] = pref[i].max(pref[i - 1])
    }
    let mut suf = nums.to_vec();
    for i in (0..n - 1).rev() {
        suf[i] = suf[i].min(suf[1 + i]);
    }
    for i in 0..n {
        if pref[i] - suf[i] <= k {
            return i as i32;
        }
    }
    -1
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
    fn basics() {}

    #[test]
    fn test() {}
}
