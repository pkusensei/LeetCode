mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
    let mut res = 0;
    let mut neg = 0;
    let mut min = i64::MAX;
    for &num in matrix.iter().flatten() {
        let num = i64::from(num);
        if num <= 0 {
            neg += 1;
        }
        min = min.min(num.abs());
        res += num.abs();
    }
    if neg & 1 == 1 {
        res - 2 * min.abs()
    } else {
        res
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
