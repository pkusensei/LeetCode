mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn has_all_codes(s: &str, k: i32) -> bool {
    let (n, k) = (s.len(), k as usize);
    if n <= k {
        return false;
    }
    let mask = (1 << k) - 1;
    let mut window = i32::from_str_radix(&s[..k], 2).unwrap_or(0);
    let mut set = std::collections::HashSet::from([window]);
    for i in 1..=n - k {
        window = (window << 1) & mask;
        window |= i32::from(s.as_bytes()[i + k - 1] == b'1');
        set.insert(window);
    }
    set.len() == 2usize.pow(k as u32)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        assert!(has_all_codes("00110110", 2));
        assert!(has_all_codes("0110", 1));
        assert!(!has_all_codes("0110", 2));
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
