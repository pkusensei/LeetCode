mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn seconds_to_remove_occurrences(mut s: String) -> i32 {
    let mut res = 0;
    // while s.contains(&"01") {
    //     res += 1;
    //     s = s.replace("01", "10");
    // }
    let mut zeros = 0;
    for b in s.bytes() {
        zeros += i32::from(b == b'0');
        if b == b'1' && zeros > 0 {
            res = (1 + res).max(zeros);
        }
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
