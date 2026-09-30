mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_depth_after_split(seq: String) -> Vec<i32> {
    let [mut open0, mut open1] = [0, 0];
    let mut res = Vec::with_capacity(seq.len());
    for b in seq.bytes() {
        if b == b'(' {
            if open0 >= open1 {
                res.push(1);
                open1 += 1;
            } else {
                res.push(0);
                open0 += 1;
            }
        } else {
            if open0 < open1 {
                open1 -= 1;
                res.push(1);
            } else {
                open0 -= 1;
                res.push(0);
            }
        }
    }
    res
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
