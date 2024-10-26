mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unique_letter_string(s: &str) -> i32 {
    let n = s.len();
    // latest two occurrences of each char
    let mut indices = [[-1, -1]; 26];
    let mut res = 0;
    for (idx, b) in s.bytes().enumerate() {
        let idx = idx as i32;
        let pos = usize::from(b - b'A');
        res += (idx - indices[pos][1]) * (indices[pos][1] - indices[pos][0]);
        indices[pos] = [indices[pos][1], idx];
    }
    res + indices
        .into_iter()
        .map(|v| (n as i32 - v[1]) * (v[1] - v[0]))
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(unique_letter_string("ABC"), 10);
        debug_assert_eq!(unique_letter_string("ABA"), 8);
        debug_assert_eq!(unique_letter_string("LEETCODE"), 92);
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
