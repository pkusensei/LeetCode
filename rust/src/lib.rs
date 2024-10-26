mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn to_goat_latin(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .enumerate()
        .map(|(i, s)| transform(s, i))
        .collect::<Vec<_>>()
        .join(" ")
}

fn transform(s: &str, idx: usize) -> String {
    const VOWELS: &str = "aeiouAEIOU";
    let mut res = vec![];
    if s.starts_with(|c| VOWELS.contains(c)) {
        res.extend(s.bytes());
    } else {
        res.extend(s[1..].bytes());
        res.push(s.as_bytes()[0]);
    }
    res.extend_from_slice(b"ma");
    res.extend(std::iter::repeat(b'a').take(1 + idx));
    String::from_utf8(res).unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            to_goat_latin(&"I speak Goat Latin"),
            "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
        );
        debug_assert_eq!(to_goat_latin(& "The quick brown fox jumped over the lazy dog"),"heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa");
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
