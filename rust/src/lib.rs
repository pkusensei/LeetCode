mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_parentheses(s: String) -> String {
    let mut st = vec![];
    let mut res = vec![];
    for b in s.bytes() {
        match b {
            b'(' => st.push(res.len()),
            b')' => {
                let top = st.pop().unwrap();
                res[top..].reverse();
            }
            _ => res.push(b),
        }
    }
    String::from_utf8(res).unwrap()
}

pub fn teleport(s: &str) -> String {
    let (s, n) = (s.as_bytes(), s.len());
    let mut pairs = vec![0; n];
    let mut st = vec![];
    for (idx, &b) in s.iter().enumerate() {
        if b == b'(' {
            st.push(idx);
        } else if b == b')' {
            let top = st.pop().unwrap();
            pairs[top] = idx;
            pairs[idx] = top;
        }
    }
    let mut idx = 0;
    let mut dir = true;
    let mut res = vec![];
    while idx < n {
        if s[idx].is_ascii_alphabetic() {
            res.push(s[idx]);
        } else {
            idx = pairs[idx];
            dir = !dir;
        }
        idx = if dir { 1 + idx } else { idx - 1 };
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
        assert_eq!(teleport("(d(bc)a)"), "abcd")
    }

    #[test]
    fn test() {}
}
