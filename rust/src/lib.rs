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
    let mut suf_min = nums.to_vec();
    for i in (0..n - 1).rev() {
        suf_min[i] = suf_min[i].min(suf_min[1 + i]);
    }
    let mut pref_max = i32::MIN;
    for i in 0..n {
        pref_max = pref_max.max(nums[i]);
        if pref_max - suf_min[i] <= k {
            return i as i32;
        }
    }
    -1
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
