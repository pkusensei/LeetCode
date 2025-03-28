mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_ways(ranges: &mut [[i32; 2]]) -> i32 {
    ranges.sort_unstable_by_key(|v| v[0]);
    let mut curr_end = ranges[0][1];
    let mut count = 1;
    for r in ranges.iter() {
        let [a, b] = [r[0], r[1]];
        if curr_end < a {
            count += 1;
        }
        curr_end = curr_end.max(b);
    }
    mod_pow(2, count, 1_000_000_007) as _
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
        assert_eq!(count_ways(&mut [[6, 10], [5, 15]]), 2);
        assert_eq!(count_ways(&mut [[1, 3], [10, 20], [2, 5], [4, 8]]), 4);
    }

    #[test]
    fn test() {}
}
