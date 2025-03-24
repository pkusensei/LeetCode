mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn is_possible(n: i32, edges: &[[i32; 2]]) -> bool {
    use itertools::Itertools;
    use std::collections::HashSet;
    let n = n as usize;
    let adj = edges.iter().fold(vec![HashSet::new(); n], |mut acc, e| {
        let [a, b] = [0, 1].map(|i| e[i] as usize - 1);
        acc[a].insert(b);
        acc[b].insert(a);
        acc
    });
    let odds: Vec<_> = adj
        .iter()
        .enumerate()
        .filter(|(_, s)| s.len() & 1 == 1)
        .collect();
    match odds.len() {
        0 => true,
        2 => {
            let a = odds[0];
            let b = odds[1];
            if !a.1.contains(&b.0) && !b.1.contains(&a.0) {
                true
            } else {
                (0..n)
                    .any(|i| i != a.0 && i != b.0 && !adj[i].contains(&a.0) && !adj[i].contains(&b.0))
            }
        }
        4 => {
            for v in odds.into_iter().permutations(4) {
                let [a, b, c, d] = v[..] else { unreachable!() };
                if !a.1.contains(&b.0) && !c.1.contains(&d.0) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
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
        assert!(is_possible(
            5,
            &[[1, 2], [2, 3], [3, 4], [4, 2], [1, 4], [2, 5]]
        ));
        assert!(is_possible(4, &[[1, 2], [3, 4]]));
        assert!(!is_possible(4, &[[1, 2], [1, 3], [1, 4]]));
    }

    #[test]
    fn test() {
        assert!(!is_possible(4, &[[1, 2], [2, 3], [3, 4], [4, 1], [1, 3]]));
        assert!(is_possible(
            17,
            &[
                [11, 12],
                [11, 8],
                [4, 2],
                [15, 6],
                [6, 11],
                [2, 11],
                [12, 16]
            ]
        ));
    }
}
