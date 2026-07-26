mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn aggregate_time_series(series1: Vec<Vec<i32>>, series2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut seen = HashSet::new();
    for v1 in &series1 {
        let i = series2.partition_point(|v2| v2[0] < v1[0]);
        let curr = series2.get(i).map(|v2| v2[1]).unwrap_or(0);
        res.push(vec![v1[0], v1[1] + curr]);
        seen.insert(v1[0]);
    }
    for v2 in series2 {
        if seen.contains(&v2[0]) {
            continue;
        }
        let i = series1.partition_point(|v1| v1[0] < v2[0]);
        let curr = series1.get(i).map(|v1| v1[1]).unwrap_or(0);
        res.push(vec![v2[0], v2[1] + curr]);
    }
    res.sort_unstable();
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
