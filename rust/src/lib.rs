mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_completing_word(s: &str, words: &[&str]) -> String {
    let table = s
        .bytes()
        .filter_map(|b| {
            if b.is_ascii_alphabetic() {
                Some(usize::from(b.to_ascii_lowercase() - b'a'))
            } else {
                None
            }
        })
        .fold([0; 26], |mut acc, i| {
            acc[i] += 1;
            acc
        });
    words
        .iter()
        .filter(|s| check(table, s))
        .min_by_key(|s| s.len())
        .map(|s| s.to_string())
        .unwrap_or_default()
}

fn check(mut table: [i32; 26], s: &str) -> bool {
    for i in s
        .bytes()
        .map(|b| usize::from(b.to_ascii_lowercase() - b'a'))
    {
        table[i] -= 1
    }
    table.into_iter().all(|v| v <= 0)
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            shortest_completing_word("1s3 PSt", &["step", "steps", "stripe", "stepple"]),
            "steps"
        );
        debug_assert_eq!(
            shortest_completing_word("1s3 456", &["looks", "pest", "stew", "show"]),
            "pest"
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
}
