mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn lexical_order(n: i32) -> Vec<i32> {
    let mut res = Vec::with_capacity(n as usize);
    let mut curr = 1;
    for _ in 0..n {
        res.push(curr);
        if 10 * curr <= n {
            curr *= 10;
        } else {
            while curr % 10 == 9 || curr >= n {
                curr /= 10
            }
            curr += 1;
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
        debug_assert_eq!(
            lexical_order(13),
            [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
        );
        debug_assert_eq!(lexical_order(2), [1, 2])
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
