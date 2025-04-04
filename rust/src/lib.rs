mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn matrix_sum_queries(n: i32, queries: &[[i32; 3]]) -> i64 {
    use std::collections::HashSet;
    let n = n as usize;
    let mut res = 0;
    let mut row_set = HashSet::new();
    let mut col_set = HashSet::new();
    for q in queries.iter().rev() {
        let [t, idx, val] = q[..] else { unreachable!() };
        let idx = idx as usize;
        let val = i64::from(val);
        if t == 0 && row_set.insert(idx) {
            res += val * (n - col_set.len()) as i64;
        } else if t == 1 && col_set.insert(idx) {
            res += val * (n - row_set.len()) as i64;
        }
    }
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
            matrix_sum_queries(3, &[[0, 0, 1], [1, 2, 2], [0, 2, 3], [1, 0, 4]]),
            23
        );
        assert_eq!(
            matrix_sum_queries(3, &[[0, 0, 4], [0, 1, 2], [1, 0, 1], [0, 2, 3], [1, 2, 1]]),
            17
        );
    }

    #[test]
    fn test() {}
}
