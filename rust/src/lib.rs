mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn remove_duplicates(s: &str, k: i32) -> String {
    let mut stack = vec![];
    for b in s.bytes() {
        if stack.iter().rev().take_while(|&&v| v == b).count() as i32 == k - 1 {
            for _ in 0..k - 1 {
                stack.pop();
            }
        } else {
            stack.push(b);
        }
    }
    String::from_utf8(stack).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(remove_duplicates("abcd", 2), "abcd");
        assert_eq!(remove_duplicates("deeedbbcccbdaa", 3), "aa");
        assert_eq!(remove_duplicates("pbbcggttciiippooaais", 2), "ps");
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
