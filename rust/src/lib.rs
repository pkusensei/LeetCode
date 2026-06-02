mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;
use itertools::Itertools;

pub fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    let land = land_start_time
        .iter()
        .zip(&land_duration)
        .map(|(st, du)| [*st, *du])
        .sorted_unstable()
        .collect_vec();
    let water = water_start_time
        .iter()
        .zip(&water_duration)
        .map(|(st, du)| [*st, *du])
        .sorted_unstable()
        .collect_vec();
    f(&land, &water).min(f(&water, &land))
}

fn f(arr1: &[[i32; 2]], arr2: &[[i32; 2]]) -> i32 {
    let end1 = arr1.iter().map(|v| v[0] + v[1]).min().unwrap();
    let i = arr2.partition_point(|v| v[0] < end1);
    let a = arr2[..i].iter().map(|v| v[1]).min();
    let b = arr2[i..].iter().map(|v| v[0] + v[1]).min();
    match [a, b] {
        [None, None] => unreachable!(),
        [None, Some(b)] => b,
        [Some(a), None] => end1 + a,
        [Some(a), Some(b)] => (end1 + a).min(b),
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
