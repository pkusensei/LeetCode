mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn generate_valid_strings(n: i32, k: i32) -> Vec<String> {
    let mut res = vec![];
    backtrack(n, k, 0, &mut String::new(), &mut res);
    res
}

fn backtrack(n: i32, k: i32, idx: i32, curr: &mut String, res: &mut Vec<String>) {
    if n == idx {
        res.push(curr.clone());
        return;
    }
    if k >= idx && !curr.ends_with('1') {
        curr.push('1');
        backtrack(n, k - idx, 1 + idx, curr, res);
        curr.pop();
    }
    curr.push('0');
    backtrack(n, k, 1 + idx, curr, res);
    curr.pop();
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
