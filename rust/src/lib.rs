mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_valid_splits(nums: &[i32]) -> i32 {
    let n = nums.len();
    let mut res = f(nums);
    for i in 0..n {
        let mut arr = nums[..i].to_vec();
        arr.extend_from_slice(&nums[1 + i..]);
        res = res.max(f(&arr));
    }
    res
}

fn f(arr: &[i32]) -> i32 {
    let n = arr.len();
    let prefix = arr.iter().fold(vec![], |mut acc, &v| {
        if let Some(&x) = acc.last() {
            acc.push(gcd(x, v));
        } else {
            acc.push(v);
        }
        acc
    });
    let mut suffix = arr.iter().rev().fold(vec![], |mut acc, &v| {
        if let Some(&x) = acc.last() {
            acc.push(gcd(x, v));
        } else {
            acc.push(v);
        }
        acc
    });
    suffix.reverse();
    let mut res = 0;
    for i in 0..n - 1 {
        if prefix[i] == suffix[1 + i] {
            res += 1
        }
    }
    res
}

const fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != 0 {
        (a, b) = (b % a, a)
    }
    b
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
    fn basics() {
        assert_eq!(max_valid_splits(&[10, 30, 15, 10]), 2);
    }

    #[test]
    fn test() {}
}
