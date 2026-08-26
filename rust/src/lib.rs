mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
    let s = s.as_bytes();
    let mut res: &[u8] = b"";
    let mut left = 0;
    let mut count = 0;
    for (right, &b) in s.iter().enumerate() {
        count += i32::from(b - b'0');
        while count == k {
            let curr = &s[left..=right];
            if res.is_empty() || curr.len() < res.len() {
                res = curr;
            } else if curr.len() == res.len() {
                res = res.min(curr);
            }
            count -= i32::from(s[left] - b'0');
            left += 1
        }
    }
    String::from_utf8(res.to_vec()).unwrap()
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
