mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_chunks_to_sorted(arr: &[i32]) -> i32 {
    // Each chunk must satisfy
    // max(chunk[n]) <= min(chunk[1+n])
    let n = arr.len();
    let (mut left_max, mut right_min) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for &num in arr.iter() {
        left_max.push(num.max(*left_max.last().unwrap_or(&0)));
    }
    for &num in arr.iter().rev() {
        right_min.push(num.min(*right_min.last().unwrap_or(&i32::MAX)));
    }
    right_min.reverse();
    1 + left_max
        .into_iter()
        .zip(right_min.into_iter().skip(1))
        .filter(|(a, b)| a <= b)
        .count() as i32
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_chunks_to_sorted(&[5, 4, 3, 2, 1]), 1);
        debug_assert_eq!(max_chunks_to_sorted(&[2, 1, 3, 4, 4]), 4);
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
