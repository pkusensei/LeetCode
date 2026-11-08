mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_mismatched_segments(s1: String, s2: String) -> Vec<Vec<String>> {
    let mut res = vec![];
    let [mut v1, mut v2] = [const { vec![] }; 2];
    for (b1, b2) in s1.bytes().zip(s2.bytes()) {
        if b1 == b2 {
            if !v1.is_empty() {
                res.push(vec![
                    String::from_utf8(v1).unwrap(),
                    String::from_utf8(v2).unwrap(),
                ]);
                v1 = vec![];
                v2 = vec![];
            }
        } else {
            v1.push(b1);
            v2.push(b2);
        }
    }
    if !v1.is_empty() {
        res.push(vec![
            String::from_utf8(v1).unwrap(),
            String::from_utf8(v2).unwrap(),
        ]);
    }
    res
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
