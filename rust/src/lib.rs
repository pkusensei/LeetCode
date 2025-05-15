mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let mut res = 0;
    let mut left = 0;
    let mut count = [0, 0];
    for (right, &b) in s.iter().enumerate() {
        count[usize::from(b - b'0')] += 1;
        while count[0] > k && count[1] > k {
            count[usize::from(s[left] - b'0')] -= 1;
            left += 1;
        }
        res += right - left + 1;
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
