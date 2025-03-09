mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn full_bloom_flowers(flowers: &[[i32; 2]], people: &[i32]) -> Vec<i32> {
    let mut map = std::collections::BTreeMap::new();
    for flo in flowers.iter() {
        *map.entry(flo[0]).or_insert(0) += 1;
        *map.entry(1 + flo[1]).or_insert(0) -= 1;
    }
    let mut curr = 0;
    for v in map.values_mut() {
        curr += *v;
        *v = curr;
    }
    people
        .iter()
        .map(|&p| map.range(..=p).next_back().map(|i| *i.1).unwrap_or(0))
        .collect()
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
            full_bloom_flowers(&[[1, 6], [3, 7], [9, 12], [4, 13]], &[2, 3, 7, 11]),
            [1, 2, 2, 2]
        );
    }

    #[test]
    fn test() {}
}
