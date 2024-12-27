mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn can_reach(arr: &[i32], start: i32) -> bool {
    let n = arr.len();
    let mut queue = std::collections::VecDeque::from([start]);
    let mut seen = vec![false; n];
    seen[start as usize] = true;
    while let Some(curr) = queue.pop_front() {
        let v = arr[curr as usize];
        if v == 0 {
            return true;
        }
        for next in [curr - v, curr + v] {
            if next >= 0 && (next as usize) < n && !seen[next as usize] {
                seen[next as usize] = true;
                queue.push_back(next);
            }
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
        assert!(can_reach(&[4, 2, 3, 0, 3, 1, 2], 5));
        assert!(can_reach(&[4, 2, 3, 0, 3, 1, 2], 0));
        assert!(!can_reach(&[3, 0, 2, 1, 2], 2));
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
