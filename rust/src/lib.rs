mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    use std::collections::VecDeque;
    let n = arr.len();
    let mut queue = VecDeque::from([start as usize]);
    let mut seen = vec![false; n];
    seen[start as usize] = true;
    while let Some(node) = queue.pop_front() {
        if arr[node] == 0 {
            return true;
        }
        for v in [
            node.checked_sub(arr[node] as usize),
            node.checked_add(arr[node] as usize),
        ] {
            if let Some(v) = v
                && v < n
                && !seen[v]
            {
                seen[v] = true;
                queue.push_back(v);
            }
        }
    }
    false
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
