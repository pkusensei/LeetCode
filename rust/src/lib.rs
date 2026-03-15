mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub const fn count_commas(n: i64) -> i64 {
    let mut curr = 1;
    let mut c = 0;
    let mut res = 0;
    while 1000 * curr <= n {
        res += 999 * curr * c;
        c += 1;
        curr *= 1000;
    }
    res += c * (n - curr + 1);
    res
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
        // 1_000..=999_999
        assert_eq!(count_commas(1_000_000), 999002);
        assert_eq!(count_commas(2019), 1020)
    }

    #[test]
    fn test() {}
}
