mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn largest_square_area(bottom_left: Vec<Vec<i32>>, top_right: Vec<Vec<i32>>) -> i64 {
    use itertools::{Itertools, izip};
    let mut res = 0;
    for [a, b] in izip!(bottom_left.iter(), top_right.iter()).array_combinations::<2>() {
        let [ax1, ay1] = a.0[..] else { unreachable!() };
        let [ax2, ay2] = a.1[..] else { unreachable!() };
        let [bx1, by1] = b.0[..] else { unreachable!() };
        let [bx2, by2] = b.1[..] else { unreachable!() };
        let dx = ax2.min(bx2) - ax1.max(bx1);
        let dy = ay2.min(by2) - ay1.max(by1);
        res = res.max(dx.min(dy).max(0));
    }
    i64::from(res).pow(2)
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
