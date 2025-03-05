mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn cells_in_range(s: String) -> Vec<String> {
    let s = s.as_bytes();
    let mut res = vec![];
    for letter in s[0]..=s[3] {
        for digit in s[1]..=s[4] {
            res.push(String::from_utf8(vec![letter, digit]).unwrap());
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
