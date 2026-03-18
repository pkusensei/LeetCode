mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn moves_to_stamp(stamp: String, target: String) -> Vec<i32> {
    use itertools::izip;
    use std::collections::{HashSet, VecDeque};

    let [slen, tlen] = [stamp.len(), target.len()];
    let [s, t] = [stamp.as_bytes(), target.as_bytes()];
    let mut res = vec![];
    let mut done = vec![false; tlen];
    let mut queue = VecDeque::new();
    let mut nodes = Vec::with_capacity(1 + tlen - slen);
    for start in 0..=tlen - slen {
        let mut fill = HashSet::new();
        let mut todo = HashSet::new();
        for (i, (a, b)) in izip!(s, &t[start..]).enumerate() {
            if a == b {
                fill.insert(start + i);
            } else {
                todo.insert(start + i);
            }
        }
        // This whole section is clear/lastly stamped
        if todo.is_empty() {
            res.push(start as i32);
            for i in start..start + slen {
                if !done[i] {
                    queue.push_back(i);
                    done[i] = true;
                }
            }
        }
        nodes.push([fill, todo]);
    }
    while let Some(idx) = queue.pop_front() {
        // All possible starts that cover `idx`
        for start in (1 + idx).saturating_sub(slen)..=idx.min(tlen - slen) {
            if nodes[start][1].remove(&idx) && nodes[start][1].is_empty() {
                res.push(start as i32); // Find the stamp!
                for &i in nodes[start][0].iter() {
                    if !done[i] {
                        done[i] = true;
                        queue.push_back(i);
                    }
                }
            }
        }
    }
    if done.iter().all(|&b| b) {
        res.reverse();
        res
    } else {
        vec![]
    }
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
