mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_of_ways(n: i32) -> i32 {
    const M: i64 = 1_000_000_007;
    let mut abc = 6;
    let mut aba = 6;
    for _ in 1..n {
        // abc
        // bcb bca bab bac
        let next_abc = (2 * aba + 2 * abc) % M;
        // aba
        // bab bac cab cac bcb
        let next_aba = (3 * aba + 2 * abc) % M;
        abc = next_abc;
        aba = next_aba;
    }
    ((abc + aba) % M) as i32
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
