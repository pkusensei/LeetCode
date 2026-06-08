mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

use std::iter::repeat_n;

#[allow(unused_imports)]
use helper::*;

pub fn pivot_array(nums: Vec<i32>, pivot: i32) -> Vec<i32> {
    let [mut res, mut big] = [const { vec![] }; 2];
    let mut count = 0;
    for &num in nums.iter() {
        match num.cmp(&pivot) {
            std::cmp::Ordering::Less => res.push(num),
            std::cmp::Ordering::Equal => count += 1,
            std::cmp::Ordering::Greater => big.push(num),
        }
    }
    res.extend(repeat_n(pivot, count));
    res.extend(big);
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
