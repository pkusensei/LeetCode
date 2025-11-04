mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn crack_safe(n: i32, k: i32) -> String {
    if n == 1 && k == 1 {
        return "0".into();
    }
    let n = n as usize;
    let start = vec![b'0'; n - 1];
    let mut res = vec![];
    dfs(&start, k, &mut HashSet::new(), &mut res);
    res.extend(start);
    String::from_utf8(res).unwrap()
}

fn dfs(node: &[u8], k: i32, seen: &mut HashSet<Vec<u8>>, res: &mut Vec<u8>) {
    for i in 0..k {
        let mut next = node.to_vec();
        next.push(i as u8 + b'0');
        if seen.insert(next.clone()) {
            dfs(&next[1..], k, seen, res);
            res.push(i as u8 + b'0');
        }
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
    fn test() {
        assert_eq!(open_lock(&["0000"], "8888"), -1);
    }
}
