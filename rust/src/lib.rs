mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn alert_names(key_name: Vec<String>, key_time: Vec<String>) -> Vec<String> {
    let mut res: Vec<_> = key_name
        .into_iter()
        .zip(key_time)
        .fold(
            std::collections::HashMap::<String, Vec<_>>::new(),
            |mut acc, (name, time)| {
                let mut it = time.split(':');
                let minutes = it.next().and_then(|s| s.parse::<i32>().ok()).unwrap() * 60
                    + it.next().and_then(|s| s.parse::<i32>().ok()).unwrap();
                acc.entry(name).or_default().push(minutes);
                acc
            },
        )
        .into_iter()
        .filter_map(|(k, mut v)| {
            v.sort_unstable();
            for w in v.windows(3) {
                if w[2] - w[0] <= 60 {
                    return Some(k.to_string());
                }
            }
            None
        })
        .collect();
    res.sort_unstable();
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {}

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
