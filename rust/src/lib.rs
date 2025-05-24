mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn string_sequence(target: String) -> Vec<String> {
    let mut res = vec![];
    let mut curr = vec![];
    for b in target.bytes() {
        let n = curr.len();
        curr.push(b'a');
        res.push(String::from_utf8(curr.clone()).unwrap());
        while curr[n] != b {
            curr[n] += 1;
            if curr[n] > b'z' {
                curr[n] = b'a'
            }
            res.push(String::from_utf8(curr.clone()).unwrap());
        }
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
