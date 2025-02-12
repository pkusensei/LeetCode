mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn merge_triplets(triplets: &[[i32; 3]], target: [i32; 3]) -> bool {
    let del: std::collections::HashSet<_> = triplets
        .iter()
        .enumerate()
        .filter_map(|(i, v)| {
            if v.iter().zip(target.iter()).any(|(a, b)| *a > *b) {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let n = triplets.len();
    for (i, &num) in target.iter().enumerate() {
        if (0..n)
            .filter(|v| !del.contains(v))
            .all(|v| triplets[v][i] != num)
        {
            return false;
        }
    }
    true
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
        assert!(merge_triplets(
            &[[2, 5, 3], [1, 8, 4], [1, 7, 5]],
            [2, 7, 5]
        ));
        assert!(!merge_triplets(&[[3, 4, 5], [4, 5, 6]], [3, 2, 5]));
        assert!(merge_triplets(
            &[[2, 5, 3], [2, 3, 4], [1, 2, 5], [5, 2, 3]],
            [5, 5, 5]
        ));
    }

    #[test]
    fn test() {}
}
