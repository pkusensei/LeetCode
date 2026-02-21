mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    // a - (x - y) = b + (x - y)
    // (x-y) = (a-b)/2
    // x = y+(a-b)/2
    let [a, b] = [&alice_sizes, &bob_sizes].map(|v| v.iter().sum::<i32>());
    let set: HashSet<_> = alice_sizes.iter().copied().collect();
    let mut res = vec![];
    for y in bob_sizes {
        let x = y + (a - b) / 2;
        if set.contains(&x) {
            res = vec![x, y];
            break;
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
