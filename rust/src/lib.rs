mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_mentions(number_of_users: i32, events: &[[&str; 3]]) -> Vec<i32> {
    use itertools::Itertools;
    use std::collections::HashMap;
    let n = number_of_users as usize;
    let mut res = vec![0; n];
    let mut offline = HashMap::new();
    for e in events
        .iter()
        .map(|e| {
            let kind = i32::from(e[0] == "MESSAGE"); // "OFFLINE"=0
            let time = e[1].parse::<i32>().unwrap();
            (time, kind, e[2])
        })
        .sorted_unstable()
    {
        offline.retain(|_, v| *v > e.0);
        if e.1 == 0 {
            let id = e.2.parse::<usize>().unwrap();
            // Could already be offline?
            *offline.entry(id).or_insert(e.0) += 60;
        } else {
            match e.2 {
                "ALL" => {
                    for v in res.iter_mut() {
                        *v += 1;
                    }
                }
                "HERE" => {
                    for i in 0..n {
                        if !offline.contains_key(&i) {
                            res[i] += 1;
                        }
                    }
                }
                _ => {
                    for id in
                        e.2.split_ascii_whitespace()
                            .map(|s| s[2..].parse::<usize>().unwrap())
                    {
                        res[id] += 1;
                    }
                }
            };
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
        assert_eq!(
            count_mentions(
                2,
                &[
                    ["MESSAGE", "10", "id1 id0"],
                    ["OFFLINE", "11", "0"],
                    ["MESSAGE", "71", "HERE"]
                ]
            ),
            [2, 2]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            count_mentions(
                3,
                &[
                    ["MESSAGE", "1", "id0 id1"],
                    ["MESSAGE", "5", "id2"],
                    ["MESSAGE", "6", "ALL"],
                    ["OFFLINE", "5", "2"]
                ]
            ),
            [2, 2, 2]
        );
    }
}
