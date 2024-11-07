mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn three_equal_parts(arr: &[i32]) -> Vec<i32> {
    let n = arr.len();
    let ones: i32 = arr.iter().sum();
    if ones == 0 {
        return vec![0, n as i32 - 1];
    }
    if ones % 3 > 0 {
        return vec![-1, -1];
    }
    let mut indices = Vec::with_capacity(2);
    let mut count = 0;
    for (idx, &num) in arr.iter().enumerate() {
        if num == 1 {
            count += 1;
        }
        if ones / 3 == count {
            indices.push(idx);
            count = 0;
        }
    }
    let zero_suffix_len = n - 1 - indices[2];
    let i = indices[0] + zero_suffix_len;
    let j = indices[1] + zero_suffix_len + 1;
    if let [Some(a), Some(b), Some(c)] = [arr, &arr[1 + i..], &arr[j..]].map(|v| {
        v.iter()
            .enumerate()
            .find_map(|(i, &v)| if v == 1 { Some(i) } else { None })
    }) {
        if arr[a..=i] == arr[b + i + 1..j] && arr[a..=i] == arr[c + j..] {
            return [i as i32, j as i32].into();
        }
    }
    vec![-1, -1]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(three_equal_parts(&[1, 0, 1, 0, 1]), [0, 3]);
        debug_assert_eq!(three_equal_parts(&[1, 1, 0, 1, 1]), [-1, -1]);
        debug_assert_eq!(three_equal_parts(&[1, 1, 0, 0, 1]), [0, 2]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            three_equal_parts(&[
                1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0,
                0, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0
            ]),
            [15, 32]
        );
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
        T1: Into<f64>,
        T2: Into<f64>,
    {
        const EP: f64 = 1e-5;
        debug_assert!((<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP);
    }
}
