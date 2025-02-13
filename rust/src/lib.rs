mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_product_difference(nums: Vec<i32>) -> i32 {
    let [mut max1, mut max2] = [i32::MIN; 2];
    let [mut min1, mut min2] = [i32::MAX; 2];
    for &num in nums.iter() {
        if num > max1 {
            max2 = max1;
            max1 = num;
        } else if num > max2 {
            max2 = num
        }
        if num < min1 {
            min2 = min1;
            min1 = num
        } else if num < min2 {
            min2 = num
        }
    }
    max1 * max2 - min1 * min2
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
