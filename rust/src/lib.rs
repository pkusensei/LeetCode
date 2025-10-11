mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn score_balance(s: String) -> bool {
    let total: i32 = s.bytes().map(|b| 1 + i32::from(b - b'a')).sum();
    if total & 1 == 1 {
        return false;
    }
    let mut curr = 0;
    for b in s.bytes() {
        curr += 1 + i32::from(b - b'a');
        if curr * 2 == total {
            return true;
        }
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
