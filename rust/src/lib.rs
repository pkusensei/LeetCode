mod helper;
mod trie;

use std::collections::{BinaryHeap, HashMap};

#[allow(unused_imports)]
use helper::*;

pub fn pyramid_transition(bottom: String, allowed: &[&str]) -> bool {
    let dict: HashMap<[u8; 2], Vec<u8>> = allowed.iter().fold(HashMap::new(), |mut acc, s| {
        let s = s.as_bytes();
        acc.entry([s[0], s[1]]).or_default().push(s[2]);
        acc
    });
    let start = bottom.into_bytes();
    let mut heap = BinaryHeap::from([Line(start)]);
    while let Some(Line(prev)) = heap.pop() {
        if prev.len() == 1 {
            return true;
        }
        let mut res = vec![];
        build(&prev, &dict, &mut vec![], &mut res);
        heap.extend(res.into_iter().map(Line));
    }
    false
}

fn build(
    prev: &[u8],
    dict: &HashMap<[u8; 2], Vec<u8>>,
    curr: &mut Vec<u8>,
    res: &mut Vec<Vec<u8>>,
) {
    let Some((window, _)) = prev.split_at_checked(2) else {
        res.push(curr.clone());
        return;
    };
    let Some(bytes) = dict.get(window) else {
        return;
    };
    for &b in bytes.iter() {
        curr.push(b);
        build(&prev[1..], dict, curr, res);
        curr.pop();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Line(Vec<u8>);

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Pop shorter first
impl Ord for Line {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.0.len().cmp(&self.0.len())
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert!(pyramid_transition(
            "BCD".into(),
            &["BCC", "CDE", "CEA", "FFF"]
        ));
        debug_assert!(!pyramid_transition(
            "AAAA".into(),
            &["AAB", "AAC", "BCD", "BBE", "DEF"]
        ));
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
