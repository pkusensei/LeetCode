mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn rotate_elements(mut nums: Vec<i32>, k: i32) -> Vec<i32> {
    use itertools::Itertools;
    let mut pos = nums.iter().copied().filter(|&v| v >= 0).collect_vec();
    if pos.is_empty() {
        return nums;
    }
    let n = pos.len();
    let k = k as usize % n;
    pos.rotate_left(k);
    let mut i = 0;
    for v in nums.iter_mut() {
        if *v >= 0 {
            *v = pos[i];
            i += 1;
        }
    }
    nums
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
