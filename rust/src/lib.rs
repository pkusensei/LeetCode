mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_abs_difference(mut arr: Vec<i32>) -> Vec<Vec<i32>> {
    arr.sort_unstable();
    let mut d = i32::MAX >> 2;
    let mut res = vec![];
    for w in arr.windows(2) {
        let curr = w[1] - w[0];
        if curr < d {
            res.clear();
            d = curr;
        }
        if curr == d {
            res.push(w);
        }
    }
    res.iter().map(|w| w.to_vec()).collect()
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
