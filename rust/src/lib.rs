mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(nums1: &[i32], nums2: &[i32]) -> i32 {
    use itertools::{Itertools, chain, izip};
    let map = chain!(nums1.iter().copied(), nums2.iter().copied()).counts();
    if map.values().any(|&f| f & 1 == 1) {
        return -1;
    }
    // 1 1 2 2   1 2 1 2
    // 3 3 4 4   3 4 3 4
    let map1 = nums1.iter().copied().counts();
    let mut res = 0;
    for (num, f) in map {
        let f1 = *map1.get(&num).unwrap_or(&0);
        if 2 * f1 != f {
            res += f.abs_diff(2 * f1) / 2
        }
    }
    res as i32 / 2
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
    fn test() {
        assert_eq!(
            min_cost(
                &[
                    18317, 39756, 32835, 66151, 32835, 67014, 33937, 76466, 55126, 36942, 46931,
                    55126, 33937, 63692, 28632, 62469, 18255
                ],
                &[
                    12430, 12430, 18255, 39756, 53076, 5307, 46931, 62469, 53076, 28632, 36942,
                    18317, 5307, 66151, 67014, 76466, 63692
                ]
            ),
            3
        )
    }
}
