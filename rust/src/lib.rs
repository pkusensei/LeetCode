mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_the_integer_zero(num1: i32, num2: i32) -> i32 {
    // num1 - n(2^p+num2) = 0
    // num1 - n*num2 = n set bits
    let [n1, n2] = [num1, num2].map(i64::from);
    for k in 1..=60 {
        if (n1 - k * n2).count_ones() <= k as u32 && k <= n1 - k * n2 {
            return k as i32;
        }
    }
    -1
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
