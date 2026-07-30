mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_pushes(word: String) -> i32 {
    let n = word.len() as i32;
    // Number of full 8-sets
    let full = n / 8;
    // Leftovers
    let rem = n % 8;
    rem * (1 + full) + full * (1 + full) * 4
    // e.g "ab_c"
    // "ab" fall in full set
    // "c" is leftover
    // Full sets take full * (1+full) / 2 * 8
    // Each leftover takes (1+full) to type, total rem*(1+full)
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
