mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_free_time(event_time: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
    use itertools::izip;
    use std::collections::BTreeMap;
    let mut prev = end_time[0];
    let mut gaps = vec![];
    let [mut left_map, mut right_map] = [const { BTreeMap::new() }; 2];
    for (&s, &e) in start_time.iter().zip(end_time.iter()).skip(1) {
        gaps.push(s - prev);
        *right_map.entry(s - prev).or_insert(0) += 1;
        prev = e;
    }
    gaps.push(event_time - prev);
    *right_map.entry(event_time - prev).or_insert(0) += 1;
    let mut left_gap = start_time[0];
    let mut res = 0;
    for (&s, &e, right_gap) in izip!(start_time.iter(), end_time.iter(), gaps) {
        let e_time = e - s;
        let v = right_map.entry(right_gap).or_insert(0);
        *v -= 1;
        if *v == 0 {
            right_map.remove(&right_gap);
        }
        res = res.max(left_gap + right_gap);
        if [&left_map, &right_map]
            .iter()
            .any(|m| m.range(e_time..).next().is_some())
        {
            res = res.max(left_gap + right_gap + e_time);
        }
        *left_map.entry(left_gap).or_insert(0) += 1;
        left_gap = right_gap;
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
