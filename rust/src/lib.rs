mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn rotated_digits(n: i32) -> i32 {
    let s = n.to_string().into_bytes();
    dfs(&s, true, false, false, &mut HashMap::new())
}

fn dfs(
    s: &[u8],
    tight: bool,
    rotated: bool,
    stay: bool,
    memo: &mut HashMap<(usize, bool, bool, bool), i32>,
) -> i32 {
    if s.is_empty() {
        return i32::from(rotated);
    }
    let k = (s.len(), tight, rotated, stay);
    if let Some(&v) = memo.get(&k) {
        return v;
    }
    let upper = if tight { s[0] } else { b'9' };
    let mut res = 0;
    for b in b'0'..=upper {
        let ntight = tight && b == upper;
        if b"018".contains(&b) {
            res += dfs(&s[1..], ntight, rotated, true, memo)
        } else if b"2569".contains(&b) {
            res += dfs(&s[1..], ntight, true, stay, memo)
        }
    }
    memo.insert(k, res);
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
        assert_eq!(rotated_digits(10), 4);
    }

    #[test]
    fn test() {}
}
