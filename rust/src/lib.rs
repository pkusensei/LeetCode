mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut res = vec![];
    dfs(n, 0, 0, &mut vec![], &mut res);
    res
}

fn dfs(n: i32, open: i32, close: i32, curr: &mut Vec<u8>, res: &mut Vec<String>) {
    if open == n {
        if close == n {
            res.push(String::from_utf8(curr.clone()).unwrap());
            return;
        }
        curr.push(b')');
        dfs(n, open, 1 + close, curr, res);
        curr.pop();
    } else {
        if open > close {
            curr.push(b')');
            dfs(n, open, 1 + close, curr, res);
            curr.pop();
        }
        curr.push(b'(');
        dfs(n, 1 + open, close, curr, res);
        curr.pop();
    }
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
