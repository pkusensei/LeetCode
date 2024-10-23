mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn unique_morse_representations(words: &[&str]) -> i32 {
    const DICT: [&str; 26] = [
        ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--",
        "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--",
        "--..",
    ];
    words
        .iter()
        .map(|s| {
            s.bytes()
                .map(|b| DICT[usize::from(b - b'a')])
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<std::collections::HashSet<_>>()
        .len() as _
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            unique_morse_representations(&["gin", "zen", "gig", "msg"]),
            2
        );
        debug_assert_eq!(unique_morse_representations(&["a"]), 1);
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
