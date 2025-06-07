mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_distance(s: String, k: i32) -> i32 {
    let mut res = 0;
    for dir in [b"NE", b"NW", b"SE", b"SW"] {
        let mut curr = 0;
        let mut k_ = k;
        for b in s.bytes() {
            if dir.contains(&b) {
                curr += 1;
            } else if k_ > 0 {
                curr += 1;
                k_ -= 1;
            } else {
                curr -= 1;
            }
            res = res.max(curr);
        }
    }
    res
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
