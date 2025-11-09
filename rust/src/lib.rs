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
        if let Some(curr) = v
            .windows(3)
            .map(|w| w[0].abs_diff(w[1]) + w[1].abs_diff(w[2]) + w[2].abs_diff(w[0]))
            .min()
            .map(|x| x as i32)
        {
            let r = res.get_or_insert(curr);
            *r = (*r).min(curr)
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
