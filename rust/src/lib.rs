mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn open_lock(deadends: &[&str], target: &str) -> i32 {
    use std::collections::{HashSet, VecDeque};
    let s = target.as_bytes();
    let tmask = to_bit_u8(s);
    let mut seen: HashSet<_> = deadends.iter().map(|s| to_bit_u8(s.as_bytes())).collect();
    if !seen.insert(to_bit_u8(&[b'0'; 4])) {
        return -1;
    }
    let mut queue = VecDeque::from([([0_i8; 4], 0)]);
    while let Some((curr, step)) = queue.pop_front() {
        let cmask = to_bit_i8(&curr);
        if cmask == tmask {
            return step;
        }
        for i in 0..4 {
            let mut next = curr;
            next[i] = (next[i] - 1).rem_euclid(10);
            let mut mask = to_bit_i8(&next);
            if seen.insert(mask) {
                queue.push_back((next, 1 + step));
            }
            next[i] = (next[i] + 2).rem_euclid(10);
            mask = to_bit_i8(&next);
            if seen.insert(mask) {
                queue.push_back((next, 1 + step));
            }
        }
    }
    -1
}

fn to_bit_u8(s: &[u8]) -> i64 {
    s.iter().fold(0, |acc, b| (acc << 10) | (1 << (b - b'0')))
}

fn to_bit_i8(s: &[i8]) -> i64 {
    s.iter()
        .fold(0, |acc, b| (acc << 10) | (1 << b.rem_euclid(10)))
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
    fn test() {
        assert_eq!(open_lock(&["0000"], "8888"), -1);
    }
}
