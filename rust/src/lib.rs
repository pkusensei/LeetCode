mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_make_subsequence(s: String, t: String) -> bool {
    let (s, n) = (s.as_bytes(), s.len());
    // Best length w/o change
    let mut no_change = 0;
    // Best length w/ only 1 change
    let mut one_change = 0;
    for target in t.bytes() {
        if one_change >= n {
            break;
        }
        // Option1
        // We already changed one letter
        // We can only extend if curr==target
        let opt1 = one_change + usize::from(s[one_change] == target);
        // Option2
        // Change could happen here regardless
        let opt2 = 1 + no_change;
        one_change = opt1.max(opt2);
        no_change += usize::from(s[no_change] == target);
    }
    one_change >= n
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
