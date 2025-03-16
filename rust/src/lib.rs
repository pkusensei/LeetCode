mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    use itertools::Itertools;
    items1
        .iter()
        .chain(items2.iter())
        .fold(std::collections::HashMap::new(), |mut acc, it| {
            let [v, w] = it[..] else { unreachable!() };
            *acc.entry(v).or_insert(0) += w;
            acc
        })
        .into_iter()
        .sorted_unstable_by_key(|&(v, _w)| v)
        .map(|(v, w)| vec![v, w])
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
    fn basics() {}

    #[test]
    fn test() {}
}
