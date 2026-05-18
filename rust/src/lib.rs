mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_jumps(arr: Vec<i32>) -> i32 {
    use std::collections::{HashMap, VecDeque};
    let n = arr.len();
    let mut map = HashMap::<_, Vec<_>>::new();
    let mut idx = 0;
    for ch in arr.chunk_by(|a, b| a == b) {
        let len = ch.len();
        if len == 1 {
            map.entry(ch[0]).or_default().push(idx);
        } else {
            map.entry(ch[0]).or_default().push(idx);
            map.entry(ch[0]).or_default().push(idx + len - 1);
        }
        idx += len;
    }
    let mut queue = VecDeque::from([(0, 0)]);
    let mut seen = vec![false; n];
    seen[0] = true;
    while let Some((node, dist)) = queue.pop_front() {
        if node == n - 1 {
            return dist;
        }
        if let Some(v) = node.checked_sub(1)
            && !seen[v]
        {
            seen[v] = true;
            queue.push_back((v, 1 + dist));
        }
        if 1 + node < n && !seen[1 + node] {
            seen[1 + node] = true;
            queue.push_back((1 + node, 1 + dist));
        }
        if let Some(v) = map.remove(&arr[node]) {
            for next in v {
                if !seen[next] {
                    seen[next] = true;
                    queue.push_back((next, 1 + dist));
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
        assert_eq!(smallest_unique_subarray(&[3, 3, 3]), 3);
        assert_eq!(smallest_unique_subarray(&[1, 1, 2, 2, 1]), 2);
    }

    #[test]
    fn test() {}
}
