mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn last_integer(n: i64) -> i64 {
    dfs(1, n, n, true, 0)
}

const fn dfs(start: i64, end: i64, count: i64, dir: bool, pow: u32) -> i64 {
    if count > 1 {
        let (a, b) = match (count & 1, dir) {
            (1, true) => (start, end),
            (0, true) => (start, end - 2_i64.pow(pow)),
            (1, false) => (start, end),
            (0, false) => (start + 2_i64.pow(pow), end),
            _ => unreachable!(),
        };
        dfs(a, b, (1 + count) / 2, !dir, 1 + pow)
    } else {
        start
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
    fn basics() {
        assert_eq!(last_integer(5), 1);
        assert_eq!(last_integer(8), 3);
        assert_eq!(last_integer(9), 9);
    }

    #[test]
    fn test() {}
}
