mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_ways(corridor: String) -> i32 {
    const M: i64 = 1_000_000_007;

    if !corridor.contains('S') {
        return 0;
    }
    let mut s_count = 0;
    let mut p_count = 0;
    let mut res = 1;
    for b in corridor.bytes() {
        if b == b'S' {
            if s_count == 2 {
                res = res * (1 + p_count) % M;
                s_count = 1;
                p_count = 0
            } else {
                s_count += 1;
            }
        } else if s_count == 2 {
            p_count += 1;
        }
    }
    if s_count & 1 == 1 { 0 } else { res as i32 }
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
