mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn judge_square_sum(c: i32) -> bool {
    let mut right = f64::from(c).sqrt().trunc() as i32;
    let mut left = 0;
    while left <= right {
        match (left * left + right * right).cmp(&c) {
            std::cmp::Ordering::Less => left += 1,
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Greater => right -= 1,
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(judge_square_sum(5));
        debug_assert!(!judge_square_sum(3));
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
