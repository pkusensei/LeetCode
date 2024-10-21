mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn custom_sort_string(order: &str, s: &str) -> String {
    let mut counts = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    let mut res = Vec::with_capacity(s.len());
    for b in order.bytes() {
        let idx = usize::from(b - b'a');
        res.extend(std::iter::repeat(b).take(counts[idx]));
        counts[idx] = 0;
    }
    for (i, c) in counts.into_iter().enumerate() {
        res.extend(std::iter::repeat(i as u8 + b'a').take(c));
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(custom_sort_string("cba", "abcd"), "cbad");
        debug_assert_eq!(custom_sort_string("bcafg", "abcd"), "bcad");
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
