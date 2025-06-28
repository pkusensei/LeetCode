mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_make_equal(nums: &[i32], k: i32) -> bool {
    solve(nums, k, 1) || solve(nums, k, -1)
}

fn solve(nums: &[i32], mut k: i32, target: i32) -> bool {
    use itertools::Itertools;
    let arr = nums
        .iter()
        .enumerate()
        .filter_map(|(i, &v)| if v != target { Some(i) } else { None })
        .collect_vec();
    if arr.len() & 1 == 1 {
        return false;
    }
    for ch in arr.chunks(2) {
        k -= (ch[1] - ch[0]) as i32;
        if k < 0 {
            return false;
        }
    }
    k >= 0
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
