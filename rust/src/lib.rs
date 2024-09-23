mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn find_all_concatenated_words_in_a_dict(words: &[&str]) -> Vec<String> {
    let mut dict = words.iter().copied().collect();
    words
        .iter()
        .filter_map(|s| {
            if word_break(s, &mut dict) {
                Some(s.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn word_break<'a>(s: &'a str, words: &mut HashSet<&'a str>) -> bool {
    if words.is_empty() {
        return false;
    }
    words.remove(s);
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..=n {
        for j in 0..i {
            if dp[j] && words.contains(&s[j..i]) {
                dp[i] = true
            }
        }
    }
    words.insert(s);
    dp[n]
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_all_concatenated_words_in_a_dict(&[
                "cat",
                "cats",
                "catsdogcats",
                "dog",
                "dogcatsdog",
                "hippopotamuses",
                "rat",
                "ratcatdogcat"
            ]),
            ["catsdogcats", "dogcatsdog", "ratcatdogcat"]
        );
        debug_assert_eq!(
            find_all_concatenated_words_in_a_dict(&["cat", "dog", "catdog"]),
            ["catdog"]
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
