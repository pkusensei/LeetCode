mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn best_tower(towers: Vec<Vec<i32>>, center: Vec<i32>, radius: i32) -> Vec<i32> {
    use itertools::Itertools;
    use std::cmp::Reverse;
    towers
        .iter()
        .filter(|t| {
            let [tx, ty] = t[..2] else { unreachable!() };
            let [cx, cy] = center[..] else { unreachable!() };
            tx.abs_diff(cx) + ty.abs_diff(cy) <= radius as u32
        })
        .sorted_unstable_by_key(|t| (Reverse(t[2]), t[0], t[1]))
        .next()
        .map(|t| t[..2].to_vec())
        .unwrap_or(vec![-1, -1])
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
