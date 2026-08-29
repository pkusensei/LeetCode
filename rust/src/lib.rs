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
    let prefix = nums.iter().fold(Vec::with_capacity(n), f);
    let mut suffix = nums.iter().rev().fold(Vec::with_capacity(n), f);
    suffix.reverse();
    let mut res = (0..n - 1).filter(|&i| prefix[i] == suffix[1 + i]).count() as i32;
    for i in 0..n - 1 {
        if prefix[i] != prefix[1 + i] {
            res = res.max(solve(&nums, 1 + i));
        }
    }
    for i in (0..n - 1).rev() {
        if suffix[i] != suffix[1 + i] {
            res = res.max(solve(&nums, i));
        }
    }
    res
}

fn f(mut acc: Vec<i32>, v: &i32) -> Vec<i32> {
    if let Some(&x) = acc.last() {
        acc.push(gcd(x, *v));
    } else {
        acc.push(*v);
    }
    acc
}

fn solve(nums: &[i32], skip: usize) -> i32 {
    let n = nums.len();
    let mut prefix = Vec::with_capacity(n);
    for (i, &num) in nums.iter().enumerate() {
        if i == skip {
            continue;
        }
        if let Some(&x) = prefix.last() {
            prefix.push(gcd(x, num));
        } else {
            prefix.push(num);
        }
    }
    let mut suffix = Vec::with_capacity(n);
    for (i, &num) in nums.iter().enumerate().rev() {
        if i == skip {
            continue;
        }
        if let Some(&x) = suffix.last() {
            suffix.push(gcd(x, num));
        } else {
            suffix.push(num);
        }
    }
    suffix.reverse();
    let mut res = 0;
    for i in 0..n - 2 {
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
