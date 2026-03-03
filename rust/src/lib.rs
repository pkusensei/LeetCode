mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::sync::LazyLock;

#[allow(unused_imports)]
use helper::*;

pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
    let [left, right] = [left, right].map(|s| s.parse::<i64>().unwrap());
    let a = left.isqrt();
    let b = right.isqrt();
    let i1 = P.partition_point(|&v| v < a);
    let i2 = P.partition_point(|&v| v <= b);
    (i2 - i1) as _
}

static P: LazyLock<Vec<i64>> = LazyLock::new(|| {
    let mut res: Vec<i64> = (1..=9).collect();
    for half in 1..10_000 {
        let a = half.to_string();
        let b = a.chars().rev().collect::<String>();
        res.push(format!("{a}{b}").parse::<i64>().unwrap());
        for d in 0..=9 {
            res.push(format!("{a}{d}{b}").parse::<i64>().unwrap());
        }
    }
    res.sort_unstable();
    res.into_iter()
        .filter(|&v| {
            let sq = v.pow(2);
            is_palindrome(sq.to_string().as_bytes())
        })
        .collect()
});

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
