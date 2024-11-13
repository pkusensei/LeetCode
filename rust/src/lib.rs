mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn pancake_sort(arr: &mut [i32]) -> Vec<i32> {
    let mut res = vec![];
    let n = arr.len();
    for len in (1..=n).rev() {
        let (idx, _) = arr[..len]
            .iter()
            .enumerate()
            .max_by_key(|(_i, &v)| v)
            .unwrap();
        if idx == len - 1 {
            continue;
        } else if idx > 0 {
            res.push(1 + idx as i32);
            arr[..=idx].reverse();
        }
        res.push(len as i32);
        arr[..len].reverse();
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // 3 [4 2 3 1]
        // 4 [1 3 2 4]
        // 2 [3 1 2 4]
        // 3 [2 1 3 4]
        // 2 [1 2 3 4]
        debug_assert_eq!(pancake_sort(&mut [3, 2, 4, 1]), [3, 4, 2, 3, 2]);
        debug_assert!(pancake_sort(&mut [1, 2, 3]).is_empty());
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
        if !((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP) {
            dbg!(a, b);
        }
    }
}
