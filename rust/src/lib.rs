mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_distance(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let map = nums
        .iter()
        .enumerate()
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (i, &v)| {
            acc.entry(v).or_default().push(i);
            acc
        });
    let mut res = None;
    for v in map.values() {
        if v.len() < 3 {
            continue;
        }
        for (i1, a) in v.iter().enumerate() {
            for (i2, b) in v.iter().enumerate().skip(1 + i1) {
                for c in v.iter().skip(1 + i2) {
                    let curr = a.abs_diff(*b) + b.abs_diff(*c) + a.abs_diff(*c);
                    let r = res.get_or_insert(curr as i32);
                    *r = (*r).min(curr as i32);
                }
            }
        }
    }
    res.unwrap_or(-1)
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
