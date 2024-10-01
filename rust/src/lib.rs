mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn check_inclusion(s1: &str, s2: &str) -> bool {
    let (n1, n2) = (s1.len(), s2.len());
    if n1 > n2 {
        return false;
    }
    let c1 = count(s1.as_bytes());
    let mut c2 = count(s2[..n1].as_bytes());
    if c1 == c2 {
        return true;
    }
    let s2 = s2.as_bytes();
    for i in 1..=n2 - n1 {
        c2[usize::from(s2[i - 1] - b'a')] -= 1;
        c2[usize::from(s2[i + n1 - 1] - b'a')] += 1;
        if c1 == c2 {
            return true;
        }
    }
    false
}

fn count(s: &[u8]) -> [i16; 26] {
    s.iter().fold([0; 26], |mut acc, &b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    })
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(check_inclusion("ab", "eidbaooo"));
        debug_assert!(!check_inclusion("ab", "eidboaoo"));
    }

    #[test]
    fn test() {
        debug_assert!(check_inclusion("adc", "dcda"))
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
