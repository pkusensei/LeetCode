mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: Vec<i32>) -> i32 {
    const fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 { b } else { gcd(b % a, a) }
    }

    let n = nums.len();
    let one = nums.iter().filter(|&&v| v == 1).count();
    if one > 0 {
        return (n - one) as i32;
    }
    let mut min = None;
    for (left, &a) in nums.iter().enumerate() {
        let mut gcd_ = a;
        for (right, &b) in nums.iter().enumerate().skip(1 + left) {
            gcd_ = gcd(gcd_, b);
            if gcd_ == 1 {
                let v = min.get_or_insert(right - left);
                *v = (*v).min(right - left)
            }
        }
    }
    min.map(|v| (n + v - 1) as i32).unwrap_or(-1)
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
