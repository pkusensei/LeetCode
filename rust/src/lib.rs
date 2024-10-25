mod helper;
mod trie;

use std::collections::HashSet;

#[allow(unused_imports)]
use helper::*;

pub fn remove_subfolders(mut folder: Vec<&str>) -> Vec<&str> {
    folder.sort_unstable_by_key(|s| s.len());
    let mut dels = HashSet::new();
    for (i1, needle) in folder.iter().enumerate() {
        if dels.contains(&i1) {
            continue;
        }
        for (i2, hay) in folder.iter().enumerate().skip(i1) {
            if dels.contains(&i2) {
                continue;
            }
            if hay.strip_prefix(needle).is_some_and(|v| v.starts_with('/')) {
                dels.insert(i2);
            }
        }
    }
    folder
        .into_iter()
        .enumerate()
        .filter_map(|(i, s)| if dels.contains(&i) { None } else { Some(s) })
        .collect()
}

fn with_trie(mut folder: Vec<&str>) -> Vec<&str> {
    folder.sort_unstable_by_key(|s| s.len());
    let mut trie = Trie::default();
    folder.into_iter().filter(|s| trie.insert(s)).collect()
}

#[derive(Debug, Clone, Default)]
struct Trie<'a> {
    nodes: std::collections::HashMap<&'a str, Trie<'a>>,
    is_end: bool,
}

impl<'a> Trie<'a> {
    fn insert(&mut self, s: &'a str) -> bool {
        let mut curr = self;
        for seg in s.split('/').skip(1) {
            curr = curr.nodes.entry(seg).or_default();
            if curr.is_end {
                return false; // has seen prefix
            }
        }
        curr.is_end = true;
        true
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            with_trie(vec!["/a", "/a/b", "/c/d", "/c/d/e", "/c/f"]),
            ["/a", "/c/d", "/c/f"],
        );
        sort_eq(with_trie(vec!["/a", "/a/b/c", "/a/b/d"]), ["/a"]);
        sort_eq(
            with_trie(vec!["/a/b/c", "/a/b/ca", "/a/b/d"]),
            ["/a/b/c", "/a/b/ca", "/a/b/d"],
        );
    }

    #[test]
    fn test() {
        sort_eq(
            with_trie(vec![
                "/aa/ab/ac/ad",
                "/aa/aq/ay",
                "/bf/bv/cd/ch/cj",
                "/bf/bg",
                "/aa/aq/ar",
                "/bf",
                "/aa/ab/aj/an/ao",
                "/aa/aq/ay/az",
                "/aa/aq/ay/bc",
                "/bf/bg/bh/bi/bj",
                "/bf/bv/bw/ca/cc",
                "/bf/bg/bh/bl",
            ]),
            [
                "/aa/ab/ac/ad",
                "/aa/ab/aj/an/ao",
                "/aa/aq/ar",
                "/aa/aq/ay",
                "/bf",
            ],
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
