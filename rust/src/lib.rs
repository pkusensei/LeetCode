mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimize_array_value(nums: &[i32]) -> i32 {
    let mut res = 0;
    let mut prefix = 0.0;
    // [.. a+1, b-1 ..]
    // 1's can "flow" to left
    // The average is the minimum of max of [..right]
    for (len, &num) in (1..).zip(nums.iter()) {
        let len = f64::from(len);
        prefix += f64::from(num);
        res = res.max((prefix / len).ceil() as i32);
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
    fn basics() {
        assert_eq!(minimize_array_value(&[3, 7, 1, 6]), 5);
        assert_eq!(minimize_array_value(&[10, 1]), 10);
    }

    #[test]
    fn test() {}
}
