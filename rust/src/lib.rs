mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_parity_alternating(nums: Vec<i32>) -> Vec<i32> {
    let a = f(&nums, 0);
    let b = f(&nums, 1);
    a.min(b)
}

fn f(nums: &[i32], mut parity: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(nums.len());
    let mut min = *nums.iter().min().unwrap();
    let mut max = *nums.iter().max().unwrap();
    let mut count = 0;
    for &num in nums.iter() {
        if num & 1 == parity {
            res.push(num);
            min = min.min(num);
            max = max.max(num);
        } else {
            if num <= min {
                res.push(1 + num);
            } else if num >= max {
                res.push(num - 1);
            }
            count += 1;
        }
        parity ^= 1;
    }
    min = *res.iter().min().unwrap();
    max = *res.iter().max().unwrap();
    vec![count, max - min]
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
