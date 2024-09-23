mod helper;
mod trie;

use std::collections::BinaryHeap;

#[allow(unused_imports)]
use helper::*;

pub fn largest_palindrome(n: i32) -> i32 {
    // [0, 9, 987, 123, 597, 677, 1218, 877, 475]
    const MOD: i64 = 1337;
    if n == 1 {
        return 9;
    }
    let mut pq = BinaryHeap::new();
    let upper = 10i64.pow(n as _) - 1;
    let lower = 10i64.pow(n as u32 - 1);
    pq.push(Pair(upper, upper - 8)); // 9*1
    pq.push(Pair(upper - 2, upper - 2)); // 7*7
    pq.push(Pair(upper - 6, upper - 6)); // 3*3
    pq.push(Pair(upper - 8, upper - 11)); // 1*9
    while let Some(Pair(a, b)) = pq.pop() {
        if is_palindrome(a * b) {
            return ((a * b) % MOD) as _;
        }
        if a >= b + 10 {
            pq.push(Pair(a - 10, b));
        }
        if a == upper || a == upper - 2 || a == upper - 6 || a == upper - 8 {
            pq.push(Pair(a, b - 10));
        }
    }
    0
}

const fn is_palindrome(num: i64) -> bool {
    let (mut temp, mut rev) = (num, 0);
    while temp > 0 {
        rev *= 10;
        rev += temp % 10;
        temp /= 10;
    }
    rev == num
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair(i64, i64);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.0 * self.1).cmp(&(other.0 * other.1))
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(largest_palindrome(2), 987);
        debug_assert_eq!(largest_palindrome(1), 9);
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
