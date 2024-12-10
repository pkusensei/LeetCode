mod dsu;
mod helper;
mod trie;

use std::{collections::HashMap, iter};

#[allow(unused_imports)]
use helper::*;

pub fn maximum_length(s: &str) -> i32 {
    let map: HashMap<u8, HashMap<usize, i32>> =
        s.as_bytes()
            .chunk_by(|a, b| a == b)
            .fold(HashMap::new(), |mut acc, w| {
                let v = acc.entry(w[0]).or_default();
                *v.entry(w.len()).or_insert(0) += 1;
                acc
            });
    let mut res = -1;
    for map in map.into_values() {
        let mut lens: Vec<_> = map
            .into_iter()
            .flat_map(|(k, count)| iter::repeat(k).take(count as usize))
            .collect();
        lens.sort_unstable();
        match (lens.pop(), lens.pop(), lens.pop()) {
            (Some(v), None, None) if v >= 3 => res = res.max(v as i32 - 2),
            (Some(1), Some(1), Some(1)) => res = res.max(1),
            (Some(a), Some(b), Some(c)) if a == b && b == c => res = res.max(a as i32),
            (Some(a), Some(b), _) if a > 1 && a == b => res = res.max(a as i32 - 1),
            (Some(a), Some(b), _) if a > 1 => res = res.max(a as i32 - 2).max(b as i32),
            _ => (),
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
        assert_eq!(maximum_length("aaaa"), 2);
        assert_eq!(maximum_length("abcdef"), -1);
        assert_eq!(maximum_length("abcaba"), 1);
    }

    #[test]
    fn test() {
        assert_eq!(maximum_length("fafff"), 1);
        assert_eq!(maximum_length("ceeeeeeeeeeeebmmmfffeeeeeeeeeeeewww"), 11);
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
