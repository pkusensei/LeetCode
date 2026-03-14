mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn solve(n: i32, mut k: i32) -> String {
    k -= 1;
    let mut res = vec![];
    let mut subtree = 1 << (n - 1);
    if k >= 3 * subtree {
        return "".to_owned();
    }
    let mut val = match k / subtree {
        0 => b'a',
        1 => b'b',
        _ => b'c',
    };
    res.push(val);
    k %= subtree;
    while res.len() < n as usize {
        subtree >>= 1;
        val = if k / subtree == 0 {
            match *res.last().unwrap() {
                b'a' => b'b',
                b'b' => b'a',
                b'c' => b'a',
                _ => unreachable!(),
            }
        } else {
            match *res.last().unwrap() {
                b'a' => b'c',
                b'b' => b'c',
                b'c' => b'b',
                _ => unreachable!(),
            }
        };
        res.push(val);
        k %= subtree;
    }
    String::from_utf8(res).unwrap()
}

pub fn get_happy_string(n: i32, mut k: i32) -> String {
    let n = n as usize;
    let mut res = Vec::with_capacity(n);
    backtrack(n, &mut k, &mut res);
    String::from_utf8(res).unwrap()
}

fn backtrack(n: usize, k: &mut i32, res: &mut Vec<u8>) -> bool {
    if res.len() == n {
        *k -= 1;
        return *k == 0;
    }
    for &b in b"abc" {
        if res.last().is_some_and(|&v| v == b) {
            continue;
        }
        res.push(b);
        if backtrack(n, k, res) {
            return true;
        }
        res.pop();
    }
    false
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
