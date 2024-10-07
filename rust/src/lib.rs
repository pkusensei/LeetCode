mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn replace_words(dictionary: &[&str], sentence: &str) -> String {
    sentence
        .split_ascii_whitespace()
        .map(|word| replace(dictionary, word).unwrap_or(word))
        .collect::<Vec<_>>()
        .join(" ")
}

fn replace<'a>(dict: &[&'a str], word: &str) -> Option<&'a str> {
    dict.iter()
        .filter(|w| word.starts_with(*w))
        .min_by_key(|w| w.len())
        .copied()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            replace_words(
                &["cat", "bat", "rat"],
                "the cattle was rattled by the battery"
            ),
            "the cat was rat by the bat"
        );
        debug_assert_eq!(
            replace_words(&["a", "b", "c"], "aadsfasf absbs bbab cadsfafs"),
            "a a b c"
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
