mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn expressive_words(s: &str, words: &[&str]) -> i32 {
    let cs: Vec<_> = s.as_bytes().chunk_by(|a, b| a == b).collect();
    let mut res = 0;
    for w in words.iter() {
        let cw: Vec<_> = w.as_bytes().chunk_by(|a, b| a == b).collect();
        if cs.len() != cw.len() {
            continue;
        }
        if cs.iter().zip(cw.iter()).all(|(a, b)| {
            a[0] == b[0] && (a.len() == b.len() || (a.len() > b.len() && a.len() >= 3))
        }) {
            res += 1;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(expressive_words("heeellooo", &["hello", "hi", "helo"]), 1);
        debug_assert_eq!(expressive_words("zzzzzyyyyy", &["zzyy", "zy", "zyy"]), 3);
    }

    #[test]
    fn test() {
        debug_assert_eq!(
            expressive_words(
                "dddiiiinnssssssoooo",
                &[
                    "dinnssoo",
                    "ddinso",
                    "ddiinnso",
                    "ddiinnssoo",
                    "ddiinso",
                    "dinsoo",
                    "ddiinsso",
                    "dinssoo",
                    "dinso"
                ]
            ),
            3
        );
    }

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
