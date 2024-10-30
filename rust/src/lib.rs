mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn k_similarity(s1: String, s2: &str) -> i32 {
    let (mut s1, s2) = (s1.into_bytes(), s2.as_bytes());
    dfs(&mut s1, s2, 0)
}

fn dfs(s1: &mut [u8], s2: &[u8], start: usize) -> i32 {
    if start == s1.len() - 1 {
        0
    } else if s1[start] == s2[start] {
        dfs(s1, s2, 1 + start)
    } else {
        let mut res = i32::MAX;
        for idx in 1 + start..s1.len() {
            if s1[idx] == s2[idx] || s2[start] != s1[idx] {
                continue;
            }
            s1.swap(start, idx);
            res = res.min(1 + dfs(s1, s2, 1 + start));
            s1.swap(start, idx);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(k_similarity("ab".into(), "ba"), 1);
        debug_assert_eq!(k_similarity("abc".into(), "bca"), 2);
        debug_assert_eq!(k_similarity("abccaacceecdeea".into(), "bcaacceeccdeaae"), 9);
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
