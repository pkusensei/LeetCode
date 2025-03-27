mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_max_difference(mut num: i32) -> i32 {
    let mut digits = vec![];
    while num > 0 {
        digits.push(num % 10);
        num /= 10;
    }
    digits.reverse();
    let mut max = 0;
    let mut mark = 9;
    for &d in digits.iter() {
        if mark == 9 && d != 9 {
            mark = d;
        }
        max = 10 * max + if d == mark { 9 } else { d };
    }
    let mut min = 0;
    mark = 0;
    for &d in digits.iter() {
        if mark == 0 && d != 0 {
            mark = d;
        }
        min = 10 * min + if d == mark { 0 } else { d };
    }
    max - min
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
