mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn num_smaller_by_frequency(queries: &[&str], words: &[&str]) -> Vec<i32> {
    let mut freqs: Vec<_> = words.iter().map(|s| freq(s)).collect();
    freqs.sort_unstable_by_key(|v| std::cmp::Reverse(*v));
    queries
        .iter()
        .map(|s| {
            let f = freq(s);
            freqs.partition_point(|&v| f < v) as i32
        })
        .collect()
}

fn freq(s: &str) -> i32 {
    s.bytes()
        .fold([0; 26], |mut acc, b| {
            acc[usize::from(b - b'a')] += 1;
            acc
        })
        .into_iter()
        .find(|&v| v > 0)
        .unwrap_or(1)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        // assert_eq!(num_smaller_by_frequency(&["cbd"], &["zaaaz"]), [1]);
        assert_eq!(
            num_smaller_by_frequency(&["bbb", "cc"], &["a", "aa", "aaa", "aaaa"]),
            [1, 2]
        );
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
        if (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() > EP {
            dbg!(a, b);
        }
    }
}
