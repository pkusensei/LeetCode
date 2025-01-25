mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut seen = std::collections::HashMap::new();
        let mut res = -1;
        for (idx, b) in s.bytes().enumerate() {
            if let Some(prev) = seen.get(&b) {
                res = res.max(idx as i32 - prev - 1);
            }
            seen.entry(b).or_insert(idx as i32);
        }
        res as _
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
    fn basics() {
        let mut fancy = Fancy::new();
        fancy.append(2); // fancy sequence: [2]
        fancy.add_all(3); // fancy sequence: [2+3] -> [5]
        fancy.append(7); // fancy sequence: [5, 7]
        fancy.mult_all(2); // fancy sequence: [5*2, 7*2] -> [10, 14]
        assert_eq!(fancy.get_index(0), 10); // return 10
        fancy.add_all(3); // fancy sequence: [10+3, 14+3] -> [13, 17]
        fancy.append(10); // fancy sequence: [13, 17, 10]
        fancy.mult_all(2); // fancy sequence: [13*2, 17*2, 10*2] -> [26, 34, 20]
        assert_eq!(fancy.get_index(0), 26); // return 26
        assert_eq!(fancy.get_index(1), 34); // return 34
        assert_eq!(fancy.get_index(2), 20); // return 20
    }

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
