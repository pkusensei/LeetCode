mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_words(words: &[&str]) -> Vec<String> {
    let rows = ["qwertyuiop", "asdfghjkl", "zxcvbnm"].map(to_bits);
    words
        .iter()
        .filter_map(|s| {
            let bits = to_bits(s);
            if rows.iter().any(|&n| n | bits == n) {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn to_bits(s: &str) -> i32 {
    s.bytes()
        .fold(0, |acc, b| acc | 1 << (b.to_ascii_lowercase() - b'a'))
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_words(&["Hello", "Alaska", "Dad", "Peace"]),
            ["Alaska", "Dad"]
        );
        debug_assert!(find_words(&["omk"]).is_empty());
        debug_assert_eq!(find_words(&["adsdf", "sfd"]), ["adsdf", "sfd"]);
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
