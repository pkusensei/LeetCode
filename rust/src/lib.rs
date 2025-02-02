mod dsu;
mod helper;
mod trie;

use std::collections::{HashMap, HashSet};

#[allow(unused_imports)]
use helper::*;

pub fn minimum_teachings(_n: i32, languages: &[&[i32]], friendships: &[[i32; 2]]) -> i32 {
    let languages: Vec<HashSet<_>> = languages
        .iter()
        .map(|v| v.iter().copied().collect())
        .collect();
    let nums: HashSet<_> = friendships
        .iter()
        .filter_map(|fr| {
            let [a, b] = [0, 1].map(|i| fr[i] as usize - 1);
            if languages[a].intersection(&languages[b]).count() == 0 {
                Some([a, b])
            } else {
                None
            }
        })
        .flatten()
        .collect();
    let mut freq = HashMap::new();
    for &num in nums.iter() {
        for &s in languages[num].iter() {
            *freq.entry(s).or_insert(0) += 1;
        }
    }
    let max = freq.into_values().max().unwrap_or(0);
    nums.len() as i32 - max
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            minimum_teachings(2, &[&[1], &[2], &[1, 2]], &[[1, 2], [1, 3], [2, 3]]),
            1
        )
    }

    #[test]
    fn test() {}
}
