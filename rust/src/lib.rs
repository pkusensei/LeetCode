mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_servers(n: i32, mut logs: Vec<Vec<i32>>, x: i32, queries: Vec<i32>) -> Vec<i32> {
    use itertools::Itertools;
    use std::collections::HashMap;

    logs.sort_unstable_by_key(|v| v[1]);
    let q_sorted = queries
        .iter()
        .enumerate()
        .map(|(i, &v)| (i, v))
        .sorted_unstable_by_key(|v| v.1)
        .collect_vec();
    let mut res = vec![0; queries.len()];
    let mut freq = HashMap::new();
    let mut left = 0;
    let mut right = 0;
    for (qi, q) in q_sorted {
        let start = q - x;
        while let Some(v) = logs.get(right)
            && v[1] <= q
        {
            *freq.entry(v[0]).or_insert(0) += 1;
            right += 1;
        }
        while let Some(v) = logs.get(left)
            && v[1] < start
        {
            let f = freq.entry(v[0]).or_insert(0);
            *f -= 1;
            if *f == 0 {
                freq.remove(&v[0]);
            }
            left += 1;
        }
        res[qi] = n - freq.len() as i32;
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
