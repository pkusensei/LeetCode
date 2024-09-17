mod helper;

use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use helper::*;

pub fn strong_password_checker(password: &str) -> i32 {
    let n = password.len();
    let s = password.as_bytes();
    let required = check_categories(s);
    if n < 6 {
        return required.max(6 - n as i32);
    }

    let mut repeats = find_repeats(password.as_bytes());
    if n <= 20 {
        let change_count: i32 = repeats.iter().map(|&n| n / 3).sum();
        return required.max(change_count);
    }

    // Any repeat requires len/3 edits, deletions or replacements
    // `3s` and `3s+2` require the same amount of edits
    let mut heap: BinaryHeap<_> = repeats.into_iter().map(|n| (Reverse(n % 3), n)).collect();
    for _ in 0..n - 20 {
        let Some((Reverse(_), n)) = heap.pop() else {
            break;
        };
        if n > 3 {
            heap.push((Reverse((n - 1) % 3), n - 1));
        }
    }
    repeats = heap.into_iter().map(|(_, n)| n).collect();
    let change_count: i32 = repeats.iter().map(|&n| n / 3).sum();
    required.max(change_count) + n as i32 - 20
}

fn check_categories(s: &[u8]) -> i32 {
    let has_lower: i32 = s.iter().any(u8::is_ascii_lowercase).into();
    let has_upper: i32 = s.iter().any(u8::is_ascii_uppercase).into();
    let has_digit: i32 = s.iter().any(u8::is_ascii_digit).into();
    3 - has_lower - has_upper - has_digit
}

fn find_repeats(s: &[u8]) -> Vec<i32> {
    let mut res = vec![];
    let mut idx = 2;
    while idx < s.len() {
        if s[idx] == s[idx - 1] && s[idx] == s[idx - 2] {
            // not 3 b.c while loop advances idx and add 1 to count
            let mut count = 2;
            while idx < s.len() && s[idx] == s[idx - 1] {
                count += 1;
                idx += 1;
            }
            res.push(count);
        }
        idx += 1
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(strong_password_checker("a"), 5);
        debug_assert_eq!(strong_password_checker("aA1"), 3);
        debug_assert_eq!(strong_password_checker("1337C0d3"), 0);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
