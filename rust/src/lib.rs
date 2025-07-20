mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
    use std::collections::HashMap;
    const M: i64 = 1_000_000_007;
    let mut map = HashMap::new();
    for p in &points {
        *map.entry(p[1]).or_insert(0) += 1;
    }
    let mut sum = 0;
    let mut res = 0;
    for v in map.values() {
        let count = v * (v - 1) / 2;
        res = (res + sum * count) % M;
        sum = (sum + count) % M;
    }
    res as i32
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
