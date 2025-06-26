mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_equal_partitions(nums: &[i32], target: i64) -> bool {
    let n = nums.len();
    for mask in 1..(1 << n) - 1 {
        let [mut v1, mut v2] = [1_i64, 1];
        for (i, &num) in nums.iter().enumerate() {
            let num = i64::from(num);
            if (mask >> i) & 1 == 1 {
                v1 *= num;
            } else {
                v2 *= num;
            }
            if v1 > target || v2 > target {
                break;
            }
        }
        if v1 == target && v2 == target {
            return true;
        }
    }
    false
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
    fn basics() {
        assert!(check_equal_partitions(&[3, 1, 6, 8, 4], 24));
    }

    #[test]
    fn test() {}
}
