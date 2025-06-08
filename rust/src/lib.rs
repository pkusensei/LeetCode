mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(n as usize);
    for d in 1..=9 {
        dfs(n, d, &mut res);
    }
    res
}

fn dfs(n: i32, curr: i32, res: &mut Vec<i32>) {
    if n < curr {
        return;
    }
    res.push(curr);
    for d in 0..=9 {
        dfs(n, curr * 10 + d, res);
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
    fn basics() {
        assert_eq!(
            lexical_order(13),
            [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        assert_eq!(lexical_order(2), [1, 2]);
    }

    #[test]
    fn test() {}
}
