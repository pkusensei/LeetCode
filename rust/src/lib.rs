mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn suggested_products(products: &mut [&str], search_word: &str) -> Vec<Vec<String>> {
    products.sort_unstable();
    let n = search_word.len();
    let mut res = Vec::with_capacity(n);
    let mut start = 0;
    for end in 1..=n {
        let prefix = &search_word[..end];
        let idx = start + products[start..].partition_point(|&v| v < prefix);
        let mut curr = vec![];
        for prod in products.iter().skip(idx).take(3) {
            if prod.starts_with(prefix) {
                curr.push(prod.to_string());
            }
        }
        res.push(curr);
        start = idx;
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert_eq!(
            suggested_products(
                &mut ["mobile", "mouse", "moneypot", "monitor", "mousepad"],
                "mouse"
            ),
            [
                vec!["mobile", "moneypot", "monitor"],
                vec!["mobile", "moneypot", "monitor"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"],
                vec!["mouse", "mousepad"]
            ]
        );
        assert_eq!(
            suggested_products(&mut ["havana"], "havana"),
            [
                ["havana"],
                ["havana"],
                ["havana"],
                ["havana"],
                ["havana"],
                ["havana"]
            ]
        )
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
