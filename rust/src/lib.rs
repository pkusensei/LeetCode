mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn divisor_substrings(num: i32, k: i32) -> i32 {
    let s = num.to_string();
    let k = k as usize;
    let n = s.len();
    let mut res = 0;
    for i in 0..=n - k {
        let val = s[i..i + k].parse::<i32>().unwrap();
        res += i32::from(val > 0 && num % val == 0);
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
    fn basics() {}

    #[test]
    fn test() {}
}
