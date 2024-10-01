mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn nearest_palindromic(n: &str) -> String {
    let num = n.parse().unwrap();
    let (a, b) = (search_left(num), search_right(num));
    if a.abs_diff(num) > b.abs_diff(num) {
        b.to_string()
    } else {
        a.to_string()
    }
}

fn search_left(num: i64) -> i64 {
    let (mut left, mut right) = (1, num);
    let mut res = 0;
    while left <= right {
        let mid = left + (right - left) / 2;
        let temp = build(mid);
        if temp < num {
            res = temp;
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    res
}

fn search_right(num: i64) -> i64 {
    let (mut left, mut right) = (num, i64::MAX);
    let mut res = 0;
    while left <= right {
        let mid = left + (right - left) / 2;
        let temp = build(mid);
        if num < temp {
            res = temp;
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    res
}

fn build(num: i64) -> i64 {
    let mut n = num;
    let mut digits = vec![];
    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }
    let n = digits.len();
    digits.reverse();
    let half: Vec<_> = if n & 1 == 1 {
        digits.iter().take(n / 2 + 1).rev().copied().collect()
    } else {
        digits.iter().take(n / 2).rev().copied().collect()
    };
    digits[n / 2..].copy_from_slice(&half);
    digits.into_iter().fold(0, |acc, d| acc * 10 + d)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(nearest_palindromic("123"), "121");
        debug_assert_eq!(nearest_palindromic("1"), "0");
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
