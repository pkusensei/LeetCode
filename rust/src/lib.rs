mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn next_greatest_letter(letters: &[char], target: char) -> char {
    let i = letters.partition_point(|&ch| ch <= target);
    letters.get(i).copied().unwrap_or(letters[0])
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(next_greatest_letter(&['c', 'f', 'j'], 'a'), 'c');
        debug_assert_eq!(next_greatest_letter(&['c', 'f', 'j'], 'c'), 'f');
        debug_assert_eq!(next_greatest_letter(&['x', 'x', 'y', 'y'], 'z'), 'x');
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
