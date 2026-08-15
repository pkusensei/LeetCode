mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(s: String) -> i32 {
    let n = s.len();
    let double = format!("{s}{s}").into_bytes();
    let mut res = i32::MAX >> 1;
    for left in 0..=n {
        let mut b = left + n - 1;
        let mut a = left;
        let mut curr = left as i32;
        while a < b {
            let d = i32::from(double[b].abs_diff(double[a]));
            curr += d.min(26 - d);
            a += 1;
            b -= 1;
        }
        res = res.min(curr);
    }
    res
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
