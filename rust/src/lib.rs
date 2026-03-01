mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(n: i32) -> i32 {
    f(n)
}

const fn f(n: i32) -> i32 {
    if n <= 2 {
        return n - 1;
    }
    if n == 3 {
        return 3;
    }
    let a = n / 2;
    let b = n - n / 2;
    a * b + f(a) + f(b)
}

// 4
// 2 2
// 1 1 1 1
//
// 5
// 2 3        6
// 1 1 1 2    3
//      1 1   1
// 1 4        4

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
