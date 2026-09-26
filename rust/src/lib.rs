mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn evaluate(s: &str, knowledge: &[[&str; 2]]) -> String {
    use std::collections::HashMap;
    let n = s.len();
    let map: HashMap<_, _> = knowledge
        .iter()
        .map(|v| (v[0].as_bytes(), v[1].as_bytes()))
        .collect();
    let mut res = vec![];
    let mut prev = n;
    for (idx, b) in s.bytes().enumerate() {
        match b {
            b'(' => prev = idx,
            b')' => {
                if let Some(v) = map.get(&s.as_bytes()[1 + prev..idx]) {
                    res.extend_from_slice(v);
                } else {
                    res.push(b'?');
                }
                prev = n;
            }
            _ if prev == n => res.push(b),
            _ => (),
        }
    }
    String::from_utf8(res).unwrap()
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
    fn basics() {
        assert_eq!(
            evaluate("(name)is(age)yearsold", &[["name", "bob"], ["age", "two"]]),
            "bobistwoyearsold"
        );
    }

    #[test]
    fn test() {}
}
