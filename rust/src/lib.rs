mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn magical_string(n: i32) -> i32 {
    let mut s = vec![1, 2, 2];
    for i in 2..n as usize {
        if s.last().is_some_and(|&v| v == 2) {
            s.extend(std::iter::repeat(1).take(s[i]));
        } else {
            s.extend(std::iter::repeat(2).take(s[i]));
        }
        if s.len() > n as usize {
            break;
        }
    }
    s.into_iter().take(n as usize).filter(|&v| v == 1).count() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(magical_string(6), 3);
        debug_assert_eq!(magical_string(1), 1);
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
