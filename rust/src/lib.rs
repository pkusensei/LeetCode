mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_score(n: i32, edges: &[[i32; 2]]) -> i64 {
    let n = i64::from(n);
    (((2 * n + 3) * n - 11) * n + 6) / 6 + 2 * i64::from(edges.len() == n as usize)
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
        assert_eq!(
            max_score(6, &[[0, 3], [4, 5], [2, 0], [1, 3], [2, 4], [1, 5]]),
            82
        );
    }

    #[test]
    fn test() {
        //  3 - 0 - 4 - 1 - 2
        assert_eq!(max_score(5, &[[2, 1], [1, 4], [4, 0], [0, 3]]), 46);
    }
}
