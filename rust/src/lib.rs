mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn split_num(mut num: i32) -> i32 {
    let mut digits = vec![];
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.sort_unstable();
    let mut a = vec![];
    let mut b = vec![];
    for (i, d) in digits.into_iter().enumerate() {
        if i & 1 == 1 {
            a.push(d);
        } else {
            b.push(d);
        }
    }
    a.iter().fold(0, |acc, d| 10 * acc + d) + b.iter().fold(0, |acc, d| 10 * acc + d)
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
