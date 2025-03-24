mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_days(days: i32, meetings: &[[i32; 2]]) -> i32 {
    let mut map = std::collections::BTreeMap::new();
    for m in meetings.iter() {
        *map.entry(m[0]).or_insert(0) += 1;
        *map.entry(1 + m[1]).or_insert(0) -= 1;
    }
    let mut curr = 0;
    let mut prev = 0;
    let mut res = 0;
    for (k, v) in map.iter() {
        if curr > 0 {
            res += k - prev;
        }
        curr += v;
        prev = *k;
    }
    days - res
}

pub fn with_sorting(days: i32, meetings: &mut [[i32; 2]]) -> i32 {
    meetings.sort_unstable();
    let mut res = 0;
    let mut last_end = 0;
    for m in meetings.iter() {
        let [start, end] = *m;
        if start > 1 + last_end {
            res += start - last_end - 1;
        }
        last_end = last_end.max(end);
    }
    res += days - last_end;
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
        assert_eq!(count_days(10, &[[5, 7], [1, 3], [9, 10]]), 2);
        assert_eq!(count_days(5, &[[2, 4], [1, 3]]), 1);
        assert_eq!(count_days(6, &[[1, 6]]), 0);

        assert_eq!(with_sorting(10, &mut [[5, 7], [1, 3], [9, 10]]), 2);
        assert_eq!(with_sorting(5, &mut [[2, 4], [1, 3]]), 1);
        assert_eq!(with_sorting(6, &mut [[1, 6]]), 0);
    }

    #[test]
    fn test() {}
}
