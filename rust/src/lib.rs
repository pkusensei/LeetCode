mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_substrings(s: &str) -> i32 {
    (0..s.len()).map(|i| count(s.as_bytes(), i)).sum()
}

fn count(s: &[u8], i: usize) -> i32 {
    let n = s.len();
    let mut res = 1;
    let (mut left, mut right) = (i.checked_sub(1), i + 1);
    while let Some(a) = left {
        if right >= n {
            break;
        }
        if s[a] == s[right] {
            res += 1;
            left = a.checked_sub(1);
            right += 1;
        } else {
            break;
        }
    }
    let (mut left, mut right) = (i, i + 1);
    while right < n {
        if s[left] == s[right] {
            res += 1;
            let Some(a) = left.checked_sub(1) else {
                break;
            };
            left = a;
            right += 1;
        } else {
            break;
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
        debug_assert_eq!(count_substrings("abc"), 3);
        debug_assert_eq!(count_substrings("aaa"), 6);
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
