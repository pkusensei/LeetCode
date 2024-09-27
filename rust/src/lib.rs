mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_lu_slength(strs: &[&str]) -> i32 {
    let mut subseqs = HashMap::new();
    let mut curr = vec![];
    for s in strs.iter() {
        all_subseqs(s.as_bytes(), &mut curr, &mut subseqs);
    }
    subseqs
        .into_iter()
        .filter_map(|(k, v)| {
            if v == 1 && !k.is_empty() {
                Some(k.len() as i32)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(-1)
}

fn all_subseqs(s: &[u8], curr: &mut Vec<u8>, subseqs: &mut HashMap<Vec<u8>, i32>) {
    if s.is_empty() {
        *subseqs.entry(curr.clone()).or_insert(0) += 1;
        return;
    }
    all_subseqs(&s[1..], curr, subseqs);
    curr.push(s[0]);
    all_subseqs(&s[1..], curr, subseqs);
    curr.pop();
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(find_lu_slength(&["aba", "cdc", "eae"]), 3);
        debug_assert_eq!(find_lu_slength(&["aaa", "aaa", "aa"]), -1);
    }

    #[test]
    fn test() {
        debug_assert_eq!(find_lu_slength(&["aabbcc", "aabbcc", "e"]), 1);
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
