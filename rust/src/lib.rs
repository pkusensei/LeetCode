mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn remove_substring(s: &str, k: i32) -> String {
    use std::iter;
    let k = k as usize;
    let mut st: Vec<(u8, usize)> = vec![];
    for ch in s.as_bytes().chunk_by(|a, b| a == b) {
        if let Some(top) = st.last_mut()
            && top.0 == ch[0]
        {
            top.1 += ch.len();
            f(&mut st, k);
        } else {
            st.push((ch[0], ch.len()));
            if ch[0] == b')' {
                f(&mut st, k);
            }
        }
    }
    st.into_iter()
        .flat_map(|v| iter::repeat_n(char::from(v.0), v.1))
        .collect()
}

fn f(st: &mut Vec<(u8, usize)>, k: usize) {
    if st.len() < 2 || st.last().is_none_or(|v| v.0 == b'(') {
        return;
    }
    let mut close = st.pop().unwrap();
    let mut open = st.pop().unwrap();
    let x = close.1.min(open.1) / k;
    close.1 -= x * k;
    open.1 -= x * k;
    if open.1 > 0 {
        st.push(open);
    }
    if close.1 > 0 {
        if let Some(top) = st.last_mut()
            && top.0 == close.0
        {
            top.1 += close.1;
            f(st, k);
        } else {
            st.push(close);
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
    fn basics() {
        assert_eq!(remove_substring("(()(", 1), "((");
        assert_eq!(remove_substring("(())", 1), "");
        assert_eq!(remove_substring("((()))()()()", 3), "()()()");
    }

    #[test]
    fn test() {
        assert_eq!(remove_substring("(()(()(()))((()", 2), "(()((()");
        assert_eq!(remove_substring(")))(()()", 1), ")))(");
    }
}
