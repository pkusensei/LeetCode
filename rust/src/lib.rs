mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_to_char(s: &str, c: char) -> Vec<i32> {
    let n = s.len();
    let mut res = Vec::with_capacity(n);
    let mut seen = -10_001;
    for (idx, ch) in s.char_indices() {
        if c == ch {
            seen = idx as i32;
        }
        res.push(idx as i32 - seen);
    }
    seen = 10_001;
    for (idx, ch) in s.char_indices().rev() {
        if c == ch {
            seen = idx as i32;
        }
        res[idx] = res[idx].min(seen - idx as i32)
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            shortest_to_char("loveleetcode", 'e'),
            [3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
        );
        debug_assert_eq!(shortest_to_char("aaab", 'b'), [3, 2, 1, 0]);
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
