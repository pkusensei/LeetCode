mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_square_area(m: i32, n: i32, mut h_fences: Vec<i32>, mut v_fences: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    const M: i64 = 1_000_000_007;
    h_fences.extend([1, m]);
    let mut hnums = HashSet::new();
    for (i, a) in h_fences.iter().enumerate() {
        for b in h_fences.iter().skip(1 + i) {
            hnums.insert(b.abs_diff(*a));
        }
    }
    v_fences.extend([1, n]);
    let mut res = 0;
    for (i, a) in v_fences.iter().enumerate() {
        for b in v_fences.iter().skip(1 + i) {
            let v = a.abs_diff(*b);
            if hnums.contains(&v) {
                res = res.max(v);
            }
        }
    }
    if res == 0 {
        -1
    } else {
        (i64::from(res).pow(2) % M) as i32
    }
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
