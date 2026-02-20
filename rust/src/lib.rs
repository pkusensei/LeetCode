mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn make_largest_special(s: String) -> String {
    String::from_utf8(dfs(s.as_bytes())).unwrap()
}

fn dfs(s: &[u8]) -> Vec<u8> {
    if s.is_empty() {
        return vec![];
    }
    let mut open = 0;
    let mut left = 0;
    let mut res = vec![];
    for (idx, &b) in s.iter().enumerate() {
        open += if b == b'1' { 1 } else { -1 };
        if open == 0 {
            let mut curr = vec![b'1'];
            curr.extend(dfs(&s[1 + left..idx]));
            curr.push(b'0');
            res.push(curr);
            left = 1 + idx;
        }
    }
    res.into_iter()
        .sorted_unstable_by(|a, b| b.cmp(a))
        .flatten()
        .collect()
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
