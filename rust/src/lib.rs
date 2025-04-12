mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_pairs(coordinates: &[[i32; 2]], k: i32) -> i32 {
    let mut map = std::collections::HashMap::new();
    let mut res = 0;
    for c in coordinates.iter() {
        let [x1, y1] = c[..] else { unreachable!() };
        for x in 0..=k {
            let y = k - x;
            let x2 = x ^ x1;
            let y2 = y ^ y1;
            res += map.get(&[x2, y2]).unwrap_or(&0);
        }
        *map.entry([x1, y1]).or_insert(0) += 1;
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
        assert_eq!(count_pairs(&[[1, 2], [4, 2], [1, 3], [5, 2]], 5), 2);
        assert_eq!(
            count_pairs(&[[1, 3], [1, 3], [1, 3], [1, 3], [1, 3]], 0),
            10
        );
    }

    #[test]
    fn test() {}
}
