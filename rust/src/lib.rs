mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn gcd_sum(nums: Vec<i32>) -> i64 {
    let n = nums.len();
    let mut prefix_gcd = Vec::with_capacity(n);
    let mut max = 0;
    for &num in nums.iter() {
        max = max.max(num);
        prefix_gcd.push(gcd(num.into(), max.into()));
    }
    prefix_gcd.sort_unstable();
    let mut left = 0;
    let mut right = n - 1;
    let mut res = 0;
    while left < right {
        res += gcd(prefix_gcd[left], prefix_gcd[right]);
        left += 1;
        right -= 1;
    }
    res
}

const fn gcd(a: i64, b: i64) -> i64 {
    if a == 0 { b } else { gcd(b % a, a) }
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
