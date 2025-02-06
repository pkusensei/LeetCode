mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(n: i32, edges: &[[i32; 2]], queries: &[i32]) -> Vec<i32> {
    let n = n as usize;
    let mut degs = vec![0; n];
    let mut edge_count = HashMap::new();
    for e in edges.iter() {
        let [e1, e2] = [0, 1].map(|i| e[i] as usize - 1);
        let a = e1.min(e2);
        let b = e1.max(e2);
        degs[a] += 1;
        degs[b] += 1;
        *edge_count.entry([a, b]).or_insert(0) += 1;
    }
        let mut data = degs.clone();
        data.sort_unstable();
        let mut res = Vec::with_capacity(queries.len());
        for &q in queries.iter() {
            let mut curr = 0;
            let mut right = n;
            // Ignore [a,b] edges
            // How many [a,b] pairs has sum of degrees >q
            for left in 0..n {
                while right > 0 && data[left] + data[right - 1] > q {
                    right -= 1;
                }
                // For this left, (n-right) pairs has degrees >q
                curr += (n - right) as i32;
                if 2 * data[left] > q {
                    curr -= 1; // no edge to self
                }
            }
            curr /= 2; // double counted
            for (&[a, b], count) in edge_count.iter() {
                // With [a,b] edges considered, this pair is invalid
                if ((degs[a] + degs[b] - count)..(degs[a] + degs[b])).contains(&q) {
                    curr -= 1;
                }
            }
            res.push(curr);
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
            count_pairs(4, &[[1, 2], [2, 4], [1, 3], [2, 3], [2, 1]], &[2, 3]),
            [6, 5]
        );
        assert_eq!(
            count_pairs(
                5,
                &[
                    [1, 5],
                    [1, 5],
                    [3, 4],
                    [2, 5],
                    [1, 3],
                    [5, 1],
                    [2, 3],
                    [2, 5]
                ],
                &[1, 2, 3, 4, 5]
            ),
            [10, 10, 9, 8, 6]
        );
    }

    #[test]
    fn test() {}
}
