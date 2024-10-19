mod helper;
mod trie;

use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use helper::*;

pub fn min_swaps_couples(row: &[i32]) -> i32 {
    let pairs: HashMap<i32, Vec<i32>> = row
        .chunks_exact(2)
        .filter_map(|ch| {
            if ch[0] / 2 != ch[1] / 2 {
                Some([ch[0] / 2, ch[1] / 2])
            } else {
                None
            }
        })
        .fold(HashMap::new(), |mut acc, ch| {
            acc.entry(ch[0]).or_default().push(ch[1]);
            acc.entry(ch[1]).or_default().push(ch[0]);
            acc
        });
    let mut seen = HashSet::new();
    let mut res = 0;
    for &k in pairs.keys() {
        let mut count = 0;
        if seen.contains(&k) {
            continue;
        }
        let mut queue = VecDeque::from([k]);
        while let Some(curr) = queue.pop_front() {
            if !seen.insert(curr) {
                continue;
            }
            count += 1;
            for &n in pairs[&curr].iter() {
                queue.push_back(n);
            }
        }
        // 12 23 31
        // 13 22 31
        // 11 22 33
        res += count - 1
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_swaps_couples(&[0, 2, 1, 3]), 1);
        debug_assert_eq!(min_swaps_couples(&[3, 2, 0, 1]), 0);
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
