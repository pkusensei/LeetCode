mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn close_strings(word1: String, word2: String) -> bool {
    let [(bytes1, count1), (bytes2, count2)] = [&word1, &word2].map(|s| {
        let (mut bytes, mut count): (Vec<_>, Vec<_>) = s
            .bytes()
            .fold(std::collections::HashMap::new(), |mut acc, b| {
                *acc.entry(b).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .unzip();
        bytes.sort_unstable();
        count.sort_unstable();
        (bytes, count)
    });
    bytes1 == bytes2 && count1 == count2
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}

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
