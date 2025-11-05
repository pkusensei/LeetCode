mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn pyramid_transition(bottom: String, allowed: Vec<String>) -> bool {
    let map = allowed
        .iter()
        .map(|s| {
            let s = s.as_bytes();
            let bot = i32::from((s[0] - b'A' + 1) << 4) | i32::from(s[1] - b'A' + 1);
            (bot, i32::from(s[2] - b'A' + 1))
        })
        .fold(HashMap::<i32, Vec<_>>::new(), |mut acc, (k, v)| {
            acc.entry(k).or_default().push(v);
            acc
        });
    let bot: Vec<_> = bottom.bytes().map(|b| i32::from(b - b'A' + 1)).collect();
    let mut seen = HashSet::from([to_bit(&bot)]);
    let mut queue = VecDeque::from([bot]);
    while let Some(curr) = queue.pop_front() {
        if curr.len() == 1 {
            return true;
        }
        let mut acc = vec![];
        backtrack(&curr, &map, &mut vec![], &mut acc);
        for v in acc {
            let mask = to_bit(&v);
            if seen.insert(mask) {
                queue.push_back(v);
            }
        }
    }
    false
}

fn backtrack(
    s: &[i32],
    map: &HashMap<i32, Vec<i32>>,
    curr: &mut Vec<i32>,
    acc: &mut Vec<Vec<i32>>,
) {
    if s.len() <= 1 {
        acc.push(curr.clone());
        return;
    }
    let k = to_bit(&s[..2]);
    if let Some(v) = map.get(&k) {
        for &b in v {
            curr.push(b);
            backtrack(&s[1..], map, curr, acc);
            curr.pop();
        }
    }
}

fn to_bit(v: &[i32]) -> i32 {
    v.iter().fold(0, |acc, x| (acc << 4) | x)
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
