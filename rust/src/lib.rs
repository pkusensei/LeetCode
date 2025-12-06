mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_points(technique1: Vec<i32>, technique2: Vec<i32>, k: i32) -> i64 {
    use itertools::{Itertools, izip};
    use std::collections::HashSet;
    let deltas: HashSet<_> = izip!(&technique1, &technique2)
        .enumerate()
        .map(|(idx, (a, b))| (a - b, idx))
        .sorted_unstable_by(|a, b| b.0.cmp(&a.0))
        .take(k as usize)
        .map(|(_v, i)| i)
        .collect();
    let mut res = 0;
    for (idx, (&a, &b)) in izip!(&technique1, &technique2).enumerate() {
        if deltas.contains(&idx) {
            res += i64::from(a);
        } else {
            res += i64::from(a.max(b))
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
