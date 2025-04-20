mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximize_square_area(m: i32, n: i32, mut h_fences: Vec<i32>, mut v_fences: Vec<i32>) -> i32 {
    use std::collections::HashSet;
    const MOD: i64 = 1_000_000_007;
    h_fences.extend([1, m]);
    v_fences.extend([1, n]);
    h_fences.sort_unstable();
    v_fences.sort_unstable();
    let mut set = HashSet::new();
    for (i, &a) in h_fences.iter().enumerate() {
        for &b in h_fences.iter().skip(1 + i) {
            set.insert(b - a);
        }
    }
    let mut max = -1;
    for (i, &a) in v_fences.iter().enumerate() {
        for &b in v_fences.iter().skip(1 + i) {
            if set.contains(&(b - a)) {
                max = max.max(i64::from(b - a));
            }
        }
    }
    if max > 0 {
        (max.pow(2) % MOD) as i32
    } else {
        -1
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
    fn basics() {}

    #[test]
    fn test() {}
}
