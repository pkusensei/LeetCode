mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn max_unique_split(s: &str) -> i32 {
    let mut res = 0;
    backtrack(s.as_bytes(), &mut HashSet::new(), &mut res);
    res
}

fn backtrack<'a>(s: &'a [u8], seen: &mut HashSet<&'a [u8]>, max: &mut i32) {
    if seen.len() + s.len() <= *max as usize {
        return;
    }
    if s.is_empty() {
        *max = (*max).max(seen.len() as i32);
        return;
    }
    for idx in 1..=s.len() {
        if seen.insert(&s[..idx]) {
            backtrack(&s[idx..], seen, max);
            seen.remove(&s[..idx]);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(max_unique_split("ababccc"), 5);
        debug_assert_eq!(max_unique_split("aba"), 2);
        debug_assert_eq!(max_unique_split("aa"), 1);
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
