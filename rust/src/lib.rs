mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_palindrome_formation(a: &str, b: &str) -> bool {
    if is_palindrome(a.bytes()) || is_palindrome(b.bytes()) {
        return true;
    }
    solve(a.as_bytes(), b.as_bytes()) || solve(b.as_bytes(), a.as_bytes())
}

fn solve(a: &[u8], b: &[u8]) -> bool {
    let n = a.len();
    let [mut left, mut right] = [0, n - 1];
    while left < right && a[left] == b[right] {
        left += 1;
        right -= 1;
    }
    is_palindrome(&a[left..=right]) || is_palindrome(&b[left..=right])
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
