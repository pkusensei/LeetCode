mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn avoid_flood(rains: &[i32]) -> Vec<i32> {
    use std::collections::{BTreeSet, HashMap};
    let n = rains.len();
    let mut res = vec![-1; n];
    let mut seen = HashMap::new();
    let mut dry = BTreeSet::new();
    for (i, &lake) in rains.iter().enumerate() {
        if lake == 0 {
            dry.insert(i);
            res[i] = 1;
        } else {
            if let Some(&prev) = seen.get(&lake) {
                let Some(&d) = dry.range(1 + prev..).next() else {
                    return vec![];
                };
                dry.remove(&d);
                res[d] = lake;
            }
            seen.insert(lake, i);
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
    fn basics() {
        assert_eq!(avoid_flood(&[1, 2, 3, 4]), [-1; 4]);
        assert_eq!(avoid_flood(&[1, 2, 0, 0, 2, 1]), [-1, -1, 2, 1, -1, -1]);
        assert_eq!(avoid_flood(&[1, 2, 0, 1, 2]), []);
    }

    #[test]
    fn test() {
        assert_eq!(avoid_flood(&[0, 1, 1]), []);
    }
}
