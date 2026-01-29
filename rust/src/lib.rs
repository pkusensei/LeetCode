mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn k_similarity(s1: &str, s2: &str) -> i32 {
    use itertools::Itertools;
    use std::collections::{HashSet, VecDeque};
    let n = s1.len();
    let [s1, s2] = [&s1, &s2].map(|s| s.bytes().map(|b| b - b'a').collect_vec());
    let [mask1, mask2] = [&s1, &s2].map(|v| to_mask(v));
    let mut seen = HashSet::from([mask1]);
    let mut queue = VecDeque::from([(s1, mask1, 0)]);
    'out: while let Some((curr_s, curr_mask, step)) = queue.pop_front() {
        if curr_mask == mask2 {
            return step;
        }
        for i1 in 0..n {
            if curr_s[i1] != s2[i1] {
                for i2 in 1 + i1..n {
                    if curr_s[i2] != s2[i2] && curr_s[i1] == s2[i2] {
                        let mut temp = curr_s.clone();
                        temp.swap(i1, i2);
                        let tmask = to_mask(&temp);
                        if seen.insert(tmask) {
                            queue.push_back((temp, tmask, 1 + step));
                            if curr_s[i2] == s2[i1] {
                                // swapped pair, best swap possible
                                continue 'out;
                            }
                        }
                    }
                }
                // swapped one slot, shortcut to next one
                continue 'out;
                // Further optimization
                // Record this idx and start from there in next loop
            }
        }
    }
    -1
}

fn to_mask(v: &[u8]) -> i64 {
    const WIDTH: i64 = 3; // 0b111==7
    v.iter().fold(0, |acc, b| (acc << WIDTH) | i64::from(*b))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {{
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        }};
    }

    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
            const _EP: f64 = 1e-5;
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        }};
    }

    #[test]
    fn basics() {}

    #[test]
    fn test() {}
}
