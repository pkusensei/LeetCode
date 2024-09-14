mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn longest_substring(s: &str, k: i32) -> i32 {
    if s.len() < k as usize {
        return 0;
    }
    if k == 1 {
        return s.len() as _;
    }
    let counts = s.bytes().fold([0; 26], |mut acc, b| {
        acc[usize::from(b - b'a')] += 1;
        acc
    });
    if counts.iter().all(|&c| c >= k || c == 0) {
        return s.len() as _;
    }
    let mut res = 0;
    let mut left = 0;
    for right in s.bytes().enumerate().filter_map(|(idx, b)| {
        let i = usize::from(b - b'a');
        if (1..k).contains(&counts[i]) {
            Some(idx)
        } else {
            None
        }
    }) {
        res = res.max(longest_substring(&s[left..right], k));
        left = right + 1;
    }
    res = res.max(longest_substring(&s[left..], k));
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(longest_substring("aaabb", 3), 3);
        debug_assert_eq!(longest_substring("ababbc", 2), 5);
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(i1: I1, i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: IntoIterator<Item = T1>,
        I2: IntoIterator<Item = T2>,
    {
        let mut v1: Vec<_> = i1.into_iter().collect();
        let mut v2: Vec<_> = i2.into_iter().collect();
        v1.sort_unstable();
        v2.sort_unstable();
        debug_assert_eq!(v1, v2);
    }
}
