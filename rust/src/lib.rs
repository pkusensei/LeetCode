mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_chunks_to_sorted(arr: &[i32]) -> i32 {
    let mut stack = vec![];
    for &num in arr.iter() {
        match stack.last() {
            None => stack.push(num),
            Some(&v) if v < num => stack.push(num),
            Some(&temp) => {
                while stack.last().is_some_and(|&v| v > num) {
                    stack.pop();
                }
                stack.push(temp);
            }
        }
    }
    stack.len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_chunks_to_sorted(&[4, 3, 2, 1, 0]), 1);
        assert_eq!(max_chunks_to_sorted(&[1, 0, 2, 3, 4]), 4);
    }

    #[test]
    fn test() {
        assert_eq!(max_chunks_to_sorted(&[1, 2, 0, 3]), 2);
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

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
