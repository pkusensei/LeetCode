mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_words(s: String) -> String {
    let mut it = s.split_ascii_whitespace();
    let first = it.next().unwrap();
    let vow = first.bytes().filter(|b| b"aeiou".contains(b)).count();
    let mut res = first.to_owned();
    for s in it {
        res.push(' ');
        if s.bytes().filter(|b| b"aeiou".contains(b)).count() == vow {
            res.extend(s.bytes().rev().map(char::from));
        } else {
            res.push_str(s);
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
    fn basics() {}

    #[test]
    fn test() {}
}
