mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_the_longest_substring(s: &str) -> i32 {
    let mut mask = 0;
    let mut prefix = HashMap::from([(mask, 0)]);
    let mut res = 0;
    for (i, b) in s.bytes().enumerate() {
        match b {
            b'a' => mask ^= 1 << 0,
            b'e' => mask ^= 1 << 1,
            b'i' => mask ^= 1 << 2,
            b'o' => mask ^= 1 << 3,
            b'u' => mask ^= 1 << 4,
            _ => (),
        }
        if let Some(v) = prefix.get(&mask) {
            // takes 2(even number) to xor to the same number
            res = res.max(i as i32 + 1 - v);
        } else {
            prefix.insert(mask, 1 + i as i32);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_the_longest_substring("eleetminicoworoep"), 13);
        debug_assert_eq!(find_the_longest_substring("leetcodeisgreat"), 5);
        debug_assert_eq!(find_the_longest_substring("bcbcbc"), 6);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
