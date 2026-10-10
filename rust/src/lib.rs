mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_product_pair(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut max = i32::MIN;
    let mut res = vec![-1, -1];
    for (i1, &num1) in nums.iter().enumerate() {
        for (i2, &num2) in nums.iter().enumerate() {
            if i1 == i2 || num1 <= num2 || num1 + num2 != target {
                continue;
            }
            let v = num1 * num2;
            max = max.max(v);
            if v == max {
                res = vec![i1 as i32, i2 as i32]
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
