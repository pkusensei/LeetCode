mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn remove_invalid_parentheses(s: String) -> Vec<String> {
    if check(s.as_bytes()) {
        return vec![s];
    }
    let mut queue = VecDeque::from([s.clone().into_bytes()]);
    let mut seen = HashSet::from([s.into_bytes()]);
    let mut res = vec![];
    let mut done = false;
    while let Some(s) = queue.pop_front() {
        if check(&s) {
            res.push(String::from_utf8(s.clone()).unwrap());
            done = true;
        }
        if done {
            continue;
        }
        for (i, b) in s.iter().enumerate() {
            if matches!(b, b'(' | b')') {
                let mut curr = s[..i].to_vec();
                curr.extend_from_slice(&s[1 + i..]);
                if seen.insert(curr.clone()) {
                    queue.push_back(curr);
                }
            }
        }
    }
    res
}

fn check(s: &[u8]) -> bool {
    let mut open = 0;
    for &b in s {
        if b == b'(' {
            open += 1
        }
        if b == b')' {
            open -= 1;
            if open < 0 {
                return false;
            }
        }
    }
    open == 0
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
