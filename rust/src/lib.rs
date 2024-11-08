mod dsu;
mod helper;
mod trie;

use std::collections::{HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn moves_to_stamp(stamp: &str, target: &str) -> Vec<i32> {
    let (n1, n2) = (stamp.len(), target.len());
    let mut queue = VecDeque::new();
    let mut done = vec![false; n2];
    let mut res = vec![];
    let mut nodes = vec![];

    for wi in 0..=n2 - n1 {
        // window [wi..wi+n1]
        let [mut made, mut todo] = [0, 1].map(|_| HashSet::new());
        // For each window, map out target against stamp
        // Find out indices' match state
        for (si, b) in stamp.bytes().enumerate() {
            if target.as_bytes()[wi + si] == b {
                made.insert(wi + si);
            } else {
                todo.insert(wi + si);
            }
        }
        // Everything is matched in this window
        // i.e this is the last stamp => search backwards
        if todo.is_empty() {
            res.push(wi as i32);
            for i in wi..wi + n1 {
                if !done[i] {
                    queue.push_back(i);
                    done[i] = true
                }
            }
        }
        // Collect all windows
        nodes.push(Node { made, todo });
    }
    while let Some(idx) = queue.pop_front() {
        // For each window that contains a match/stamped idx
        for wi in (idx + 1).saturating_sub(n1)..=idx.min(n2 - n1) {
            if nodes[wi].todo.remove(&idx) && nodes[wi].todo.is_empty() {
                // This window could be totally matched
                // i.e one step backwards
                res.push(wi as i32);
                for &i in nodes[wi].made.iter() {
                    if !done[i] {
                        done[i] = true;
                        queue.push_back(i);
                    }
                }
            }
        }
    }
    if done.into_iter().any(|b| !b) {
        return vec![];
    }
    res.reverse();
    res
}

#[derive(Debug, Clone)]
struct Node {
    made: HashSet<usize>,
    todo: HashSet<usize>,
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(moves_to_stamp("abc", "ababc"), [1, 0, 2]);
        debug_assert_eq!(moves_to_stamp("abca", "aabcaca"), [2, 3, 0, 1]);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
