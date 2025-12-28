mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_all_one_multiple(k: i32) -> i32 {
    use std::collections::HashSet;
    if k & 1 == 0 {
        return -1;
    }
    let mut num = 1;
    let mut res = 1;
    let mut seen = HashSet::new();
    while seen.insert(num) {
        if num % k == 0 {
            return res;
        } else {
            num = num * 10 % k + 1;
            res += 1;
        }
    }
    -1
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
        assert_eq!(min_all_one_multiple(3), 3);
        assert_eq!(min_all_one_multiple(7), 6);
    }

    #[test]
    fn test() {}
}
