mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(colors: Vec<i32>) -> i32 {
    let mut min_idx = [None; 101];
    let mut max_idx = [None; 101];
    for (i, &c) in colors.iter().enumerate() {
        min_idx[c as usize].get_or_insert(i);
        max_idx[c as usize] = Some(i);
    }
    let mut res = 0;
    for (c1, i1) in min_idx.iter().enumerate() {
        let Some(i1) = i1 else {
            continue;
        };
        for (c2, i2) in max_idx.iter().enumerate() {
            if let Some(i2) = i2
                && c1 != c2
            {
                res = res.max(i2.abs_diff(*i1));
            }
        }
    }
    res as i32
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
