mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn get_max_repetitions(s1: &str, n1: i32, s2: &str, n2: i32) -> i32 {
    let (len1, len2) = (s1.len(), s2.len());
    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
    let (mut i1, mut i2) = (0, 0);
    let mut dict = HashMap::new();
    let total = (n1 as usize) * len1;
    while i1 < total {
        if s1[i1 % len1] == s2[i2 % len2] {
            if let Some(&(p1, p2)) = dict.get(&(i1 % len1, i2 % len2)) {
                let (period1, period2) = (i1 - p1, i2 - p2);
                let count = (total - i1) / period1;
                i1 += count * period1;
                i2 += count * period2;
                if i1 >= total {
                    break; // skips i2+=1
                }
            } else {
                dict.insert((i1 % len1, i2 % len2), (i1, i2));
            }
            i2 += 1;
        }
        i1 += 1;
    }
    (i2 / len2 / n2 as usize) as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(get_max_repetitions("acb", 4, "ab", 2), 2);
        debug_assert_eq!(get_max_repetitions("acb", 1, "acb", 1,), 1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(get_max_repetitions("baba", 11, "baab", 1), 7);
        debug_assert_eq!(get_max_repetitions("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", 1000000, "a", 1),100000000);
    }

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
