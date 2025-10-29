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

pub fn evaluate(expression: String) -> i32 {
    dfs(&expression, &HashMap::new())
}

fn dfs(s: &str, env: &HashMap<&str, i32>) -> i32 {
    if let Ok(v) = s.parse() {
        return v;
    }
    if s.as_bytes()[0].is_ascii_alphabetic() {
        return *env.get(s).unwrap_or(&0);
    }
    let mut curr = env.clone();
    let Some((left, right)) = s.split_once(' ') else {
        return 0;
    };
    let toks = split(&right[..right.len() - 1]);
    match left {
        "(add" => dfs(toks[0], &curr) + dfs(toks[1], &curr),
        "(mult" => dfs(toks[0], &curr) * dfs(toks[1], &curr),
        "(let" => {
            for ch in toks.chunks_exact(2) {
                let v = dfs(ch[1], &curr);
                curr.insert(ch[0], v);
            }
            dfs(toks.last().unwrap_or(&""), &curr)
        }
        _ => unreachable!(),
    }
}

fn split(s: &str) -> Vec<&str> {
    let mut res = vec![];
    let mut left = 0;
    let mut open = 0;
    for (right, b) in s.bytes().enumerate() {
        match b {
            b'(' => open += 1,
            b')' => open -= 1,
            _ => (),
        }
        if open == 0 && b.is_ascii_whitespace() {
            res.push(&s[left..right]);
            left = 1 + right;
        }
    }
    if !s[left..].is_empty() {
        res.push(&s[left..]);
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
