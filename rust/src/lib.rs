mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_palindrome(s: &str) -> String {
    if s.is_empty() {
        return "".to_uppercase();
    }
    let p_len = kmp(s);
    s.chars()
        .rev()
        .take(s.len() - p_len)
        .chain(s.chars())
        .collect()
}

fn kmp(s: &str) -> usize {
    let temp: Vec<_> = s
        .bytes()
        .chain(std::iter::once(b'#'))
        .chain(s.bytes().rev())
        .collect();
    let n = temp.len();
    let mut failure = vec![0; n];
    for (i, &curr) in temp.iter().enumerate().skip(1) {
        let mut j = failure[i - 1];
        while j > 0 && curr != temp[j] {
            j = failure[j - 1];
        }
        failure[i] = j + usize::from(curr == temp[j])
    }
    failure.into_iter().last().unwrap()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(shortest_palindrome("aacecaaa"), "aaacecaaa");
        debug_assert_eq!(shortest_palindrome("abcd"), "dcbabcd");
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
