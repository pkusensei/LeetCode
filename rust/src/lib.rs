mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn total_score(hp: i32, damage: &[i32], requirement: &[i32]) -> i64 {
    use itertools::izip;
    let n = damage.len();
    let mut prefix = Vec::with_capacity(1 + n);
    prefix.push(0);
    // for d in damage.iter() {
    //     prefix.push(i64::from(*d) + prefix.last().unwrap_or(&0));
    // }
    // hp - sum(damage[left..=right]) >= req
    // hp - (pref[1+right]-pref[left]) >= req
    // pref[left] >= req - hp + pref[1+right]
    let mut res = 0;
    for (right, (d, req)) in izip!(damage.iter(), requirement.iter()).enumerate() {
        prefix.push(i64::from(*d) + prefix.last().unwrap_or(&0));
        let req = i64::from(*req);
        let need = prefix[1 + right] + req - i64::from(hp);
        let left = prefix.partition_point(|&v| v < need);
        if left <= right {
            res += 1 + right - left;
        }
    }
    res as i64
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
    fn basics() {
        assert_eq!(total_score(11, &[3, 6, 7], &[4, 2, 5]), 3);
        assert_eq!(total_score(2, &[10000, 1], &[1, 1]), 1);
    }

    #[test]
    fn test() {
        assert_eq!(
            total_score(
                6,
                &[1, 10000, 1, 10000, 1, 10000],
                &[1, 10000, 1, 10000, 1, 10000]
            ),
            3
        );
        assert_eq!(total_score(1, &[1], &[2]), 0);
    }
}
