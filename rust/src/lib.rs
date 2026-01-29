mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn car_fleet(target: i32, position: &[i32], speed: &[i32]) -> i32 {
    use itertools::{Itertools, izip};
    use std::cmp::Reverse;
    let cars = izip!(position.iter(), speed.iter())
        .map(|(&p, &s)| (p, s))
        .sorted_unstable_by_key(|(p, _s)| Reverse(*p))
        .collect_vec();
    let mut st = vec![];
    for (pos, sp) in cars {
        // This car takes `curr` time to reach `target`
        let curr = f64::from(target - pos) / f64::from(sp);
        // But the car before this one takes longer
        // A crash/catch-up happens
        // Both cars run at same speed and arrive at same time now
        // In this case `curr` is irrelevant
        if st.last().is_some_and(|&top| top >= curr) {
            continue;
        }
        st.push(curr);
    }
    st.len() as i32
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
