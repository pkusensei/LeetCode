mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn earliest_finish_time(
    land_start_time: Vec<i32>,
    land_duration: Vec<i32>,
    water_start_time: Vec<i32>,
    water_duration: Vec<i32>,
) -> i32 {
    f(
        &land_start_time,
        &land_duration,
        &water_start_time,
        &water_duration,
    )
    .min(f(
        &water_start_time,
        &water_duration,
        &land_start_time,
        &land_duration,
    ))
}

fn f(start1: &[i32], dur1: &[i32], start2: &[i32], dur2: &[i32]) -> i32 {
    let earliest = start1
        .iter()
        .zip(dur1)
        .map(|(s, d)| s + d)
        .min()
        .unwrap_or(0);
    let mut res = i32::MAX;
    for (&s, &d) in start2.iter().zip(dur2) {
        let curr = s.max(earliest) + d;
        res = res.min(curr);
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
