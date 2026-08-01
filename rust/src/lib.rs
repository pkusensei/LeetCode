mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_width(planks: Vec<i32>) -> i32 {
    use itertools::Itertools;
    use std::collections::HashMap;
    let freq = planks.iter().copied().counts();
    let mut map = HashMap::new();
    for (i, (&a, &f1)) in freq.iter().enumerate() {
        *map.entry(a).or_insert(0) += f1;
        *map.entry(2 * a).or_insert(0) += f1 / 2;
        for (&b, &f2) in freq.iter().skip(1 + i) {
            *map.entry(a + b).or_insert(0) += f1.min(f2);
        }
    }
    map.into_values().max().unwrap() as i32
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
