mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_score(scores: &[i32], edges: &[[i32; 2]]) -> i32 {
    use std::{cmp::Reverse, collections::BinaryHeap};
    let n = scores.len();
    let mut heaps = vec![BinaryHeap::with_capacity(4); n];
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        heaps[a].push((Reverse(scores[b]), b));
        heaps[b].push((Reverse(scores[a]), a));
        if heaps[a].len() > 3 {
            heaps[a].pop();
        }
        if heaps[b].len() > 3 {
            heaps[b].pop();
        }
    }
    let mut res = -1;
    for e in edges.iter() {
        let [a, b] = [0, 1].map(|i| e[i] as usize);
        for &(Reverse(s1), n1) in heaps[a].iter() {
            for &(Reverse(s2), n2) in heaps[b].iter() {
                if n1 != n2 && n1 != b && n2 != a {
                    res = res.max(scores[a] + scores[b] + s1 + s2);
                }
            }
        }
    }
    res
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

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {{
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
    fn basics() {
        assert_eq!(
            maximum_score(
                &[5, 2, 9, 8, 4],
                &[[0, 1], [1, 2], [2, 3], [0, 2], [1, 3], [2, 4]]
            ),
            24
        );
        assert_eq!(
            maximum_score(&[9, 20, 6, 4, 11, 12], &[[0, 3], [5, 3], [2, 4], [1, 3]]),
            -1
        );
    }

    #[test]
    fn test() {}
}
