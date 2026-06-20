mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_lights(lights: &[i32]) -> i32 {
    let n = lights.len();
    let mut intervals = vec![];
    for (i, &v) in lights.iter().enumerate() {
        if v > 0 {
            let curr = [i.saturating_sub(v as usize), (i + v as usize).min(n - 1)];
            intervals.push(curr);
        }
    }
    if intervals.is_empty() {
        return (n as i32 + 2) / 3;
    }
    intervals.sort_unstable();
    let mut arr = vec![];
    let [mut start, mut end] = intervals[0];
    for item in intervals[1..].iter() {
        let [curr_s, curr_e] = *item;
        if curr_s <= end {
            end = end.max(curr_e);
        } else {
            arr.push([start, end]);
            start = curr_s;
            end = curr_e;
        }
    }
    arr.push([start, end]);
    let mut prev = 0;
    let mut res = 0;
    for [start, end] in arr {
        res += (start - prev + 2) / 3;
        prev = 1 + end;
    }
    res += (n - prev + 2) / 3;
    res as i32
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
        assert_eq!(min_lights(&[0, 0, 0, 2, 0]), 1);
    }

    #[test]
    fn test() {}
}
