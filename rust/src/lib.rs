mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn valid_strings(n: i32) -> Vec<String> {
    let mut res = vec![];
    backtrack(n, &mut Vec::with_capacity(n as usize), &mut res);
    res
}

fn backtrack(n: i32, curr: &mut Vec<u8>, res: &mut Vec<String>) {
    if n == 0 {
        res.push(curr.iter().map(|&b| char::from(b)).collect());
        return;
    }
    curr.push(b'1');
    backtrack(n - 1, curr, res);
    curr.pop();
    if curr.last().is_none_or(|&v| v == b'1') {
        curr.push(b'0');
        backtrack(n - 1, curr, res);
        curr.pop();
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
