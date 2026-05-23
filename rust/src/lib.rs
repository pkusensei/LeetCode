mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_operations(nums: &[i32], k: i32) -> i32 {
    let mut res = i32::MAX;
    for x in 0..k {
        for y in 0..k {
            if x == y {
                continue;
            }
            let mut curr = 0;
            for (i, &num) in nums.iter().enumerate() {
                let val = if i & 1 == 0 { x } else { y };
                let d = (num % k - val).abs();
                curr += d.min(k - d);
            }
            res = res.min(curr);
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
    fn test() {
        assert_eq!(min_operations(&[63, 36, 77, 19], 4), 3)
    }
}
