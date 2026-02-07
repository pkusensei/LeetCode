mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn prime_palindrome(n: i32) -> i32 {
    if (8..=11).contains(&n) {
        return 11;
    }
    for a in 1..=100_000 {
        let mut s = a.to_string();
        let rev = s.chars().rev().skip(1).collect::<String>();
        s.push_str(&rev);
        if let Ok(num) = s.parse()
            && num >= n
            && check(num)
        {
            return num;
        }
    }
    -1
}

const fn check(num: i32) -> bool {
    if num <= 2 || num & 1 == 0 {
        return num == 2;
    }
    let mut p = 3;
    while p * p <= num {
        if num % p == 0 {
            return false;
        }
        p += 2;
    }
    true
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

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
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
