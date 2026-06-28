mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn filter_occupied_intervals(
    mut occupied_intervals: Vec<[i32; 2]>,
    mut free_start: i32,
    free_end: i32,
) -> Vec<Vec<i32>> {
    occupied_intervals.sort_unstable();
    let mut start = occupied_intervals[0][0];
    let mut end = occupied_intervals[0][1];
    let mut merged = vec![];
    for v in &occupied_intervals[1..] {
        let [curr_s, curr_e] = v[..] else {
            unreachable!()
        };
        if 1 + end >= curr_s {
            end = end.max(curr_e)
        } else {
            merged.push([start, end]);
            start = curr_s;
            end = curr_e;
        }
    }
    merged.push([start, end]);
    let mut res = vec![];
    for v in merged {
        let [start, end] = v;
        if free_end < start || end < free_start {
            res.push(vec![start, end]);
        } else if free_start <= start && free_end < end {
            res.push(vec![1 + free_end, end]);
        } else if start < free_start && end <= free_end {
            res.push(vec![start, free_start - 1]);
        } else if start < free_end && free_end < end {
            res.push(vec![start, free_start - 1]);
            res.push(vec![1 + free_end, end]);
        }
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
    fn basics() {
        assert_eq!(
            filter_occupied_intervals(vec![[1, 3]], 2, 2),
            [[1, 1], [3, 3]]
        );
    }

    #[test]
    fn test() {
        assert_eq!(
            filter_occupied_intervals(vec![[50, 77], [44, 61], [48, 50]], 71, 77),
            [[44, 70]]
        );
        assert_eq!(
            filter_occupied_intervals(
                vec![[31, 40], [75, 92], [44, 46], [50, 51], [43, 47]],
                45,
                52
            ),
            [[31, 40], [43, 44], [75, 92]]
        );
    }
}
