mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_distance_after_queries(n: i32, queries: &[[i32; 2]]) -> Vec<i32> {
    use std::collections::BTreeMap;
    let mut dist = n - 1;
    let mut map = BTreeMap::<i32, i32>::new();
    let mut res = vec![];
    for q in queries.iter() {
        let [a, b] = q[..] else {
            continue;
        };
        if map.range(..=a).next_back().is_some_and(|(_k, &v)| b <= v) {
            // [1..4] in map
            // [2..3] is covered
            res.push(dist);
            continue;
        }
        // [2..3] in map
        // [1..4] is being added
        let del: Vec<_> = map
            .range(a..)
            .filter_map(|(&k, &v)| if v <= b { Some(k) } else { None })
            .collect();
        for k in del {
            let span = map.remove(&k).map(|v| v - k).unwrap_or(0);
            dist += span - 1;
        }
        map.insert(a, b);
        dist += 1 - (b - a);
        res.push(dist);
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
            shortest_distance_after_queries(5, &[[2, 4], [0, 2], [0, 4]]),
            [3, 2, 1]
        );
        assert_eq!(
            shortest_distance_after_queries(4, &[[0, 3], [0, 2]]),
            [1, 1]
        );
    }

    #[test]
    fn test() {}
}
