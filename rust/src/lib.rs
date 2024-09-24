mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn license_key_formatting(s: &str, k: i32) -> String {
    s.bytes()
        .filter_map(|b| {
            if b.is_ascii_alphanumeric() {
                Some(b.to_ascii_uppercase())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .rchunks(k as _)
        .map(|ch| std::str::from_utf8(ch).unwrap())
        .rev()
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(license_key_formatting("5F3Z-2e-9-w", 4), "5F3Z-2E9W");
        debug_assert_eq!(license_key_formatting("2-5g-3-J", 2), "2-5G-3J");
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
}
