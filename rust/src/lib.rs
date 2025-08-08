mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn character_replacement(s: &str, k: i32) -> i32 {
    (b'A'..=b'Z')
        .map(|t| find(s.as_bytes(), k, t))
        .max()
        .unwrap_or(0)
}

fn find(s: &[u8], k: i32, target: u8) -> i32 {
    let mut res = 0;
    let mut count = k;
    let mut left = 0;
    for (right, &b) in s.iter().enumerate() {
        count -= i32::from(b != target);
        while left <= right && count < 0 {
            count += i32::from(s[left] != target);
            left += 1;
        }
        res = res.max(right + 1 - left);
    }
    res as i32
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
    fn basics() {
        assert_eq!(character_replacement("ABAB", 2), 4);
        assert_eq!(character_replacement("AABABBA", 1), 4);
    }

    #[test]
    fn test() {}
}
