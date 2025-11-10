mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_transform(start: String, result: String) -> bool {
    let s1 = start.as_bytes();
    let s2 = result.as_bytes();
    let n = s1.len();
    if n != s2.len() {
        return false;
    }
    let mut i1 = 0;
    let mut i2 = 0;
    while i1 < n && i2 < n {
        while s1.get(i1).is_some_and(|&v| v == b'X') {
            i1 += 1;
        }
        while s2.get(i2).is_some_and(|&v| v == b'X') {
            i2 += 1;
        }
        if i1 >= n || i2 >= n {
            break;
        }
        if s1[i1] != s2[i2] {
            return false;
        }
        if s1[i1] == b'L' && i1 < i2 {
            return false;
        }
        if s1[i1] == b'R' && i1 > i2 {
            return false;
        }
        i1 += 1;
        i2 += 1;
    }
    s1[i1..].iter().chain(&s2[i2..]).all(|&v| v == b'X')
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
