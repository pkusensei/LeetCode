mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn answer_string(word: String, num_friends: i32) -> String {
    if num_friends <= 1 {
        return word;
    }
    let (s, n) = (word.as_bytes(), word.len());
    let len = n + 1 - num_friends as usize;
    let mut res = "".as_bytes();
    for idx in 0..n {
        let curr = &s[idx..(idx + len).min(n)];
        res = res.max(curr);
    }
    String::from_utf8(res.to_vec()).unwrap()
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
