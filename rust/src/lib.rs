mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    let a = f(tops[0], &tops, &bottoms).min(f(tops[0], &bottoms, &tops));
    let b = f(bottoms[0], &tops, &bottoms).min(f(bottoms[0], &bottoms, &tops));
    match [a, b] {
        [None, None] => -1,
        [None, Some(x)] | [Some(x), None] => x,
        [Some(a), Some(b)] => a.min(b),
    }
}

fn f(target: i32, a: &[i32], b: &[i32]) -> Option<i32> {
    let mut res = 0;
    for (&a, &b) in a.iter().zip(b) {
        if a != target {
            if b != target {
                return None;
            }
            res += 1
        }
    }
    Some(res)
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
