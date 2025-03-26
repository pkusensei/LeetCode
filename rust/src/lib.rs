mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
    nums.iter()
        .flat_map(|&v| {
            let mut num = v;
            let mut res = vec![];
            while num > 0 {
                res.push(num % 10);
                num /= 10;
            }
            res.reverse();
            res
        })
        .collect()
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
