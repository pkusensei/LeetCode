mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_sum_distinct_triplet(x: Vec<i32>, y: Vec<i32>) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (&n1, &n2) in x.iter().zip(y.iter()) {
        let v = map.entry(n1).or_insert(n2);
        *v = (*v).max(n2);
    }
    if map.len() < 3 {
        -1
    } else {
        map.into_values()
            .sorted_unstable_by(|a, b| b.cmp(a))
            .take(3)
            .sum()
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
