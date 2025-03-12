mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn strong_password_checker_ii(password: String) -> bool {
    password.len() >= 8
        && password.bytes().any(|v| v.is_ascii_uppercase())
        && password.bytes().any(|v| v.is_ascii_lowercase())
        && password.bytes().any(|v| v.is_ascii_digit())
        && password.bytes().any(|v| b"!@#$%^&*()-+".contains(&v))
        && password.as_bytes().windows(2).all(|w| w[0] != w[1])
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
