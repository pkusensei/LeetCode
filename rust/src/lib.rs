mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn lex_smallest(s: &str) -> String {
    let n = s.len();
    let mut res = s.to_string();
    for k in 1..=n {
        let mut curr = Vec::with_capacity(n);
        curr.extend(s.bytes().take(k).rev());
        curr.extend(s.bytes().skip(k));
        let v = String::from_utf8(curr.clone()).unwrap();
        if v < res {
            res = v;
        }
        curr.clear();
        curr.extend(s.bytes().take(n - k));
        curr.extend(s.bytes().skip(n - k).rev());
        let v = String::from_utf8(curr).unwrap();
        if v < res {
            res = v;
        }
    }
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
        assert_eq!(lex_smallest("abba"), "aabb");
    }

    #[test]
    fn test() {}
}
