mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_weight(mut pizzas: Vec<i32>) -> i64 {
    pizzas.sort_unstable();
    let n = pizzas.len() / 4;
    let odd = (1 + n) / 2;
    let mut res = 0;
    for _ in 0..odd {
        res += i64::from(pizzas.pop().unwrap_or(0))
    }
    for _ in 0..n / 2 {
        pizzas.pop();
        res += i64::from(pizzas.pop().unwrap_or(0))
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
