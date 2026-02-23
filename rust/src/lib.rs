mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn has_all_codes(s: String, k: i32) -> bool {
    use std::collections::HashSet;
    let n = s.len();
    let k = k as usize;
    if n < k {
        return false;
    }
    let full = 2_u32.pow(k as u32) - 1;
    let mut curr = u32::from_str_radix(&s[..k], 2).unwrap_or(0);
    let mut seen = HashSet::from([curr]);
    for b in s.as_bytes()[k..].iter() {
        curr = (curr << 1) & full;
        curr |= u32::from(b - b'0');
        seen.insert(curr);
    }
    seen.len() == 2_usize.pow(k as u32)
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
