mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn get_sneaky_numbers(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len() as i32 - 2;
    let mut xor = 0;
    for &num in &nums {
        xor ^= num;
    }
    for i in 0..n {
        xor ^= i;
    }
    let low_bit = xor & (-xor);
    let [mut a, mut b] = [0, 0];
    for num in nums {
        if num & low_bit == 0 {
            a ^= num
        } else {
            b ^= num
        }
    }
    for i in 0..n {
        if i & low_bit == 0 { a ^= i } else { b ^= i }
    }
    [a, b].into()
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
        assert_eq!(delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
    }

    #[test]
    fn test() {
        assert_eq!(delete_and_earn(vec![1, 6, 3, 3, 8, 4, 8, 10, 1, 3]), 43);
    }
}
