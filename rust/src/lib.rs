mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn are_sentences_similar(s1: &str, s2: &str) -> bool {
    if s1 == s2 {
        return true;
    }
    let [it1, it2] = [s1, s2].map(|s| s.split_ascii_whitespace());
    let prefix = it1
        .clone()
        .zip(it2.clone())
        .take_while(|(a, b)| a == b)
        .count();
    let suffix = it1
        .clone()
        .rev()
        .zip(it2.clone().rev())
        .take_while(|(a, b)| a == b)
        .count();
    let n = it1.count().min(it2.count());
    (prefix > 0 || suffix > 0) && prefix + suffix >= n
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(are_sentences_similar("My name is Haley", "My Haley"));
        debug_assert!(!are_sentences_similar("of", "A lot of words"));
        debug_assert!(are_sentences_similar("Eating right now", "Eating"));
    }

    #[test]
    fn test() {
        debug_assert!(!are_sentences_similar(
            "eTUny i b R UFKQJ EZx JBJ Q xXz",
            "eTUny i R EZx JBJ xXz"
        ));
        debug_assert!(are_sentences_similar("a", "a aa a"))
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
