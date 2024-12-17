mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn repeat_limited_string(s: &str, repeat_limit: i32) -> String {
    let mut heap: std::collections::BinaryHeap<_> = s
        .bytes()
        .fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
        .into_iter()
        .enumerate()
        .filter(|(_, b)| *b > 0)
        .map(|(i, v)| (i as u8 + b'a', v))
        .collect();
    let mut res = vec![];
    let k = repeat_limit as usize;
    while let Some((byte, count)) = heap.pop() {
        if res.len() >= k && res.iter().rev().take(k).all(|&v| v == byte) {
            let Some((b, c)) = heap.pop() else {
                break;
            };
            res.push(b);
            if c > 1 {
                heap.push((b, c - 1));
            }
            heap.push((byte, count));
        } else {
            res.push(byte);
            if count > 1 {
                heap.push((byte, count - 1));
            }
        }
    }
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(repeat_limited_string("cczazcc", 3), "zzcccac");
        assert_eq!(repeat_limited_string("aababab", 2), "bbabaa");
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
