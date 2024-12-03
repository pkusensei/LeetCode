mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn max_rep_opt1(text: &str) -> i32 {
    let count: Vec<_> = text
        .as_bytes()
        .chunk_by(|&a, &b| a == b)
        .map(|w| (w[0], w.len()))
        .collect();
    let map = text.bytes().fold(HashMap::new(), |mut acc, b| {
        *acc.entry(b).or_insert(0) += 1;
        acc
    });
    let a = count
        .windows(3)
        .filter_map(|w| {
            if w[0].0 == w[2].0 && w[1].1 == 1 {
                Some((w[0].0, w[0].1 + w[2].1))
            } else {
                None
            }
        })
        .max_by_key(|(_b, c)| *c)
        .map(|(b, c)| extend(&map, b, c))
        .unwrap_or(0);
    let freq = count.iter().max_by_key(|v| v.1).map(|v| v.1).unwrap_or(0);
    let mut b = 0;
    for v in count.iter().filter(|v| v.1 == freq) {
        b = b.max(extend(&map, v.0, v.1));
    }
    a.max(b) as i32
}

fn extend(map: &HashMap<u8, usize>, b: u8, c: usize) -> usize {
    if map.get(&b).is_some_and(|&v| v > c) {
        1 + c
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(max_rep_opt1("ababa"), 3);
        assert_eq!(max_rep_opt1("aaabaaa"), 6);
        assert_eq!(max_rep_opt1("aaaaa"), 5);
    }

    #[test]
    fn test() {
        assert_eq!(max_rep_opt1("aabba"), 3);
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
