mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_good_integers(n: i32) -> Vec<i32> {
    use itertools::Itertools;
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for a in 1..=1000_i32 {
        if a.pow(3) > n {
            break;
        }
        for b in a..=1000 {
            let val = a.pow(3) + b.pow(3);
            if val <= n {
                *map.entry(val).or_insert(0) += 1;
            } else {
                break;
            }
        }
    }
    map.into_iter()
        .filter_map(|(v, f)| if f >= 2 { Some(v) } else { None })
        .sorted_unstable()
        .collect()
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
