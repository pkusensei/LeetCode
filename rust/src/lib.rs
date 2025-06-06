mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_valid_pair(s: String) -> String {
    let f = |b| usize::from(b - b'0');
    let freq = s.bytes().fold([0; 10], |mut acc, b| {
        acc[f(b)] += 1;
        acc
    });
    for w in s.as_bytes().windows(2) {
        if w[0] != w[1] && f(w[0]) == freq[f(w[0])] && f(w[1]) == freq[f(w[1])] {
            return String::from_utf8(w.to_vec()).unwrap();
        }
    }
    String::new()
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
