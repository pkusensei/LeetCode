mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_stach(s: &str) -> i32 {
    let mut st = vec![0];
    for b in s.bytes() {
        if b == b'(' {
            st.push(0);
        } else {
            let last = st.pop().unwrap();
            let top = st.last_mut().unwrap();
            *top += if last == 0 { 1 } else { last * 2 };
        }
    }
    st[0]
}

pub fn score_of_parentheses(s: String) -> i32 {
    dfs(s.as_bytes())
}

fn dfs(s: &[u8]) -> i32 {
    if s.is_empty() {
        return 0;
    }
    if s == b"()" {
        return 1;
    }
    let mut open = 0;
    for (idx, &b) in s.iter().enumerate() {
        open += if b == b'(' { 1 } else { -1 };
        if open == 0 {
            if idx == s.len() - 1 {
                return 2 * dfs(&s[1..idx]);
            } else {
                return dfs(&s[..=idx]) + dfs(&s[1 + idx..]);
            }
        }
    }
    unreachable!()
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
