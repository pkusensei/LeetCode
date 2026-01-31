mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_by_type(s: String) -> String {
    let mut s = s.into_bytes();
    let [mut v1, mut v2] = [vec![], vec![]];
    for &b in s.iter() {
        if b.is_ascii_alphabetic() {
            v1.push(b);
        } else {
            v2.push(b);
        }
    }
    for b in s.iter_mut() {
        if b.is_ascii_alphabetic() {
            *b = v1.pop().unwrap()
        } else {
            *b = v2.pop().unwrap()
        }
    }
    String::from_utf8(s).unwrap()
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
