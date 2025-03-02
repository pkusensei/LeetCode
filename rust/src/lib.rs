mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn earliest_full_bloom(plant_time: &[i32], grow_time: &[i32]) -> i32 {
    let mut nums: Vec<_> = plant_time
        .iter()
        .zip(grow_time.iter())
        .map(|(&p, &g)| [p, g])
        .collect();
    nums.sort_unstable_by(|a, b| b[1].cmp(&a[1]).then(a[0].cmp(&b[0])));
    let mut res = 0;
    let mut plant = 0;
    for &[p, g] in nums.iter() {
        plant += p;
        res = res.max(plant + g);
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
        assert_eq!(earliest_full_bloom(&[1, 4, 3], &[2, 3, 1]), 9);
        assert_eq!(earliest_full_bloom(&[1, 2, 3, 2], &[2, 1, 2, 1]), 9);
        assert_eq!(earliest_full_bloom(&[1], &[1]), 2);
    }

    #[test]
    fn test() {}
}
