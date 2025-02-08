mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

// (a1^a2) & (b1^b2) = (a1&b1) ^ (a1&b2) ^ (a2&b1) ^ (a2&b2)
// (a1+a2) * (b1+b2)
pub fn get_xor_sum(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    arr1.iter().fold(0, |acc, &num| acc ^ num) & arr2.iter().fold(0, |acc, &num| acc ^ num)
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
