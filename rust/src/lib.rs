mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_binary_substrings(s: &str) -> i32 {
    s.as_bytes()
        .chunk_by(|a, b| a == b)
        .map(|ch| ch.len())
        .collect::<Vec<_>>()
        .windows(2)
        .map(|w| w[0].min(w[1]) as i32)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(count_binary_substrings("00110011"), 6);
        debug_assert_eq!(count_binary_substrings("10101"), 4);
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
