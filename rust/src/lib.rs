mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_longest_word(s: &str, dictionary: &mut [&str]) -> String {
    dictionary.sort_unstable_by(|a, b| b.len().cmp(&a.len()).then(a.cmp(b)));
    for word in dictionary.iter() {
        if is_subseq(s.as_bytes(), word.as_bytes()) {
            return word.to_string();
        }
    }
    String::new()
}

fn is_subseq(hay: &[u8], needle: &[u8]) -> bool {
    if hay.len() < needle.len() {
        return false;
    }
    let (mut i1, mut i2) = (0, 0);
    while i1 < hay.len() && i2 < needle.len() {
        if hay[i1] == needle[i2] {
            i2 += 1
        }
        i1 += 1;
    }
    i2 == needle.len()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_longest_word("abpcplea", &mut ["ale", "apple", "monkey", "plea"]),
            "apple"
        );
        debug_assert_eq!(find_longest_word("abpcplea", &mut ["a", "b", "c"]), "a");
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_longest_word("aaa", &mut ["aaa", "aa", "a"]), "aaa");
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
