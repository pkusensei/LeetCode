mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(mut arr: Vec<i32>, mut brr: Vec<i32>, k: i64) -> i64 {
    let num1: i64 = arr
        .iter()
        .zip(&brr)
        .map(|(a, b)| i64::from(a.abs_diff(*b)))
        .sum();
    arr.sort_unstable();
    brr.sort_unstable();
    let num2 = k + arr
        .iter()
        .zip(&brr)
        .map(|(a, b)| i64::from(a.abs_diff(*b)))
        .sum::<i64>();
    num1.min(num2)
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
    fn basics() {}

    #[test]
    fn test() {}
}
