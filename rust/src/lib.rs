mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_price(mut prices: Vec<i32>, mut discounts: Vec<i32>) -> f64 {
    use itertools::{EitherOrBoth, Itertools};
    use std::cmp::Reverse;
    prices.sort_unstable_by_key(|&v| Reverse(v));
    discounts.sort_unstable_by_key(|&v| Reverse(v));
    let mut res = 0.0;
    for v in prices.iter().zip_longest(discounts) {
        match v {
            EitherOrBoth::Both(&p, d) => {
                let [p, d] = [p, d].map(f64::from);
                res += p * (100.0 - d) / 100.0;
            }
            EitherOrBoth::Left(&p) => res += f64::from(p),
            EitherOrBoth::Right(_) => break,
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
