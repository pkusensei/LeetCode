mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_mentions(number_of_users: i32, mut events: Vec<Vec<String>>) -> Vec<i32> {
    use std::{
        cmp::Reverse,
        collections::{BinaryHeap, HashSet},
    };
    let n = number_of_users as usize;
    let mut on: HashSet<_> = (0..n).collect();
    let mut off = BinaryHeap::new();
    events.sort_unstable_by(|a, b| {
        let [v1, v2] = [a, b].map(|v| v[1].parse::<i32>().unwrap_or(100_000));
        v1.cmp(&v2).then(b[0].cmp(&a[0]))
    });
    let mut res = vec![0; n];
    let mut all = 0;
    for e in events {
        let time = e[1].parse::<i32>().unwrap_or(100_000);
        while off.peek().is_some_and(|&(Reverse(t), _)| t <= time) {
            let (_, id) = off.pop().unwrap();
            on.insert(id);
        }
        if e[0] == "MESSAGE" {
            match e[2].as_str() {
                "ALL" => all += 1,
                "HERE" => {
                    for &i in &on {
                        res[i] += 1;
                    }
                }
                _ => {
                    for s in e[2].split_ascii_whitespace() {
                        let id = s[2..].parse::<usize>().unwrap_or(0);
                        res[id] += 1;
                    }
                }
            }
        } else {
            let id = e[2].parse::<usize>().unwrap_or(100);
            on.remove(&id);
            off.push((Reverse(time + 60), id));
        }
    }
    for v in res.iter_mut() {
        *v += all;
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
