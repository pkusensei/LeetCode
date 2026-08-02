mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

// x/y <= a/b
// even*b <= odd*a
pub fn count_ratio_subarrays(nums: Vec<i32>, a: i32, b: i32) -> i32 {
    let mut res = 0;
    for left in 0..nums.len() {
        let [mut even, mut odd] = [0, 0];
        for &v in &nums[left..] {
            if v & 1 == 1 {
                odd += 1
            } else {
                even += 1
            }
            res += i32::from(odd > 0 && even * b <= odd * a);
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
