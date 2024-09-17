mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn character_replacement(s: &str, k: i32) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    let (mut left, mut right) = (0, 0);
    let mut letters = [0; 26];
    let mut max_count = 1;
    while right < n && right - left < (max_count + k) as usize {
        let i = usize::from(s[right] - b'A');
        letters[i] += 1;
        max_count = max_count.max(letters[i]);
        right += 1
    }
    while right < n {
        let i = usize::from(s[right] - b'A');
        right += 1;
        letters[i] += 1;
        let temp = letters[i];
        if temp > max_count {
            max_count = temp;
            // expand window size
            continue;
        }
        letters[usize::from(s[left] - b'A')] -= 1;
        left += 1
    }
    letters.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(character_replacement("ABAB", 2), 4);
        debug_assert_eq!(character_replacement("AABABBA", 1), 4);
    }

    #[test]
    fn test() {
        debug_assert_eq!(character_replacement("AAAA", 0), 4);
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
