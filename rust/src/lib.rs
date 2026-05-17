mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_kth_roots(l: i32, r: i32, k: i32) -> i32 {
    if k == 1 {
        return if l == 0 { 1 + r } else { r - l.max(1) + 1 };
    }
    if l == 0 {
        f(r, k) + 1
    } else {
        f(r, k) - f(l - 1, k)
    }
}

fn f(num: i32, k: i32) -> i32 {
    if num < 0 {
        return 0;
    }
    let mut root = num.isqrt();
    let mut kk = k / 2;
    while kk > 2 {
        kk /= 2;
        root = root.isqrt();
    }
    while root.checked_pow(k as u32).is_none_or(|v| v > num) {
        root -= 1;
    }
    root
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
    fn basics() {
        assert_eq!(count_kth_roots(1, 9, 3), 2);
        assert_eq!(count_kth_roots(8, 30, 2), 3);
    }

    #[test]
    fn test() {}
}
