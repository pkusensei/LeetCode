mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_buses_to_destination(routes: Vec<Vec<i32>>, source: i32, target: i32) -> i32 {
    use std::collections::{HashMap, HashSet, VecDeque};
    if source == target {
        return 0;
    }
    let stop_buses =
        routes
            .iter()
            .enumerate()
            .fold(HashMap::<_, Vec<usize>>::new(), |mut acc, (i, r)| {
                for &stop in r {
                    acc.entry(stop).or_default().push(i);
                }
                acc
            });
    let Some(src_buses) = stop_buses.get(&source) else {
        return -1;
    };
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    for &b in src_buses {
        queue.push_back((b, 1));
        seen.insert(b);
    }
    while let Some((bus, num)) = queue.pop_front() {
        for &stop in &routes[bus] {
            if stop == target {
                return num;
            }
            if let Some(v) = stop_buses.get(&stop) {
                for &next in v {
                    if seen.insert(next) {
                        queue.push_back((next, 1 + num));
                    }
                }
            }
        }
    }
    -1
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
        assert_eq!(
            latest_day_to_cross(
                3,
                3,
                &[
                    [1, 2],
                    [2, 1],
                    [3, 3],
                    [2, 2],
                    [1, 1],
                    [1, 3],
                    [2, 3],
                    [3, 2],
                    [3, 1]
                ]
            ),
            3
        );
    }

    #[test]
    fn test() {}
}
