mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_decodings(s: &str) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let s = s.as_bytes();
    let mut first: i64 = 1;
    let mut second: i64 = match s[0] {
        b'*' => 9,
        b'0' => 0,
        _ => 1,
    };
    for w in s.windows(2) {
        let (b1, b2) = (w[0], w[1]);
        let temp = second;
        if b2 == b'*' {
            second = (9 * second) % MOD;
            second = match b1 {
                b'1' => (second + 9 * first) % MOD,
                b'2' => (second + 6 * first) % MOD,
                b'*' => (second + 15 * first) % MOD,
                _ => second,
            };
        } else {
            second = if b2 != b'0' { second } else { 0 };
            second = match b1 {
                b'1' => (second + first) % MOD,
                b'2' if b2 <= b'6' => (second + first) % MOD,
                b'*' if b2 <= b'6' => (second + 2 * first) % MOD,
                b'*' => (second + first) % MOD,
                _ => second,
            };
        }
        first = temp;
    }
    second as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(num_decodings("*"), 9);
        debug_assert_eq!(num_decodings("1*"), 18);
        debug_assert_eq!(num_decodings("2*"), 15);
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
