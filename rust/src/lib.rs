mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_cost(colors: String, needed_time: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut res = 0;
    for ch in colors.as_bytes().chunk_by(|a, b| a == b) {
        if ch.len() == 1 {
            left += 1;
        } else {
            let mut sum = 0;
            let mut max = 0;
            let right = left + ch.len();
            for c in needed_time[left..right].iter() {
                sum += c;
                max = max.max(*c);
            }
            res += sum - max;
            left = right;
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
