mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;
use std::collections::HashMap;

pub fn count_servers(n: i32, mut logs: Vec<[i32; 2]>, x: i32, queries: Vec<i32>) -> Vec<i32> {
    logs.sort_unstable_by_key(|v| v[1]); // [id, time]
    let qn = queries.len();
    let qis = (0..qn)
        .sorted_unstable_by_key(|&i| queries[i])
        .collect_vec();
    let mut res = vec![0; qn];
    let [mut left, mut right] = [0, 0];
    let mut map = HashMap::new();
    for qi in qis {
        let start = queries[qi] - x;
        let end = queries[qi];
        while logs.get(right).is_some_and(|v| v[1] <= end) {
            *map.entry(logs[right][0]).or_insert(0) += 1;
            right += 1;
        }
        while logs.get(left).is_some_and(|v| v[1] < start) {
            let id = logs[left][0];
            *map.entry(id).or_insert(0) -= 1;
            left += 1;
            if map[&id] == 0 {
                map.remove(&id);
            }
        }
        res[qi] = n - map.len() as i32;
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
    fn basics() {
        assert_eq!(
            count_servers(3, vec![[1, 3], [2, 6], [1, 5]], 5, vec![10, 11]),
            [1, 2]
        );
        assert_eq!(
            count_servers(3, vec![[2, 4], [2, 1], [1, 2], [3, 1]], 2, vec![3, 4]),
            [0, 1]
        );
    }

    #[test]
    fn test() {}
}
