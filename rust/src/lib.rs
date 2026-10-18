mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_selection(
    mut available_intervals: Vec<[i32; 2]>,
    mut target_intervals: Vec<[i32; 2]>,
) -> i32 {
    target_intervals.sort_unstable();
    let mut target = vec![];
    let [mut start, mut end] = [target_intervals[0][0], target_intervals[0][1]];
    for v in &target_intervals[1..] {
        let [s, e] = v[..] else { unreachable!() };
        if s <= end {
            end = end.max(e)
        } else {
            target.push([start, end]);
            start = s;
            end = e;
        }
    }
    target.push([start, end]);
    let n = available_intervals.len();
    available_intervals.sort_unstable();
    let mut res = 0;
    let mut idx = 0; // idx in available
    let mut prev_end = 0;
    for [start, end] in target {
        if start == end {
            if prev_end < start {
                let mut rightmost = 0;
                while idx < n && available_intervals[idx][0] <= start {
                    rightmost = rightmost.max(available_intervals[idx][1]);
                    idx += 1;
                }
                if rightmost < start {
                    return -1;
                }
                prev_end = rightmost;
                res += 1;
            }
        } else {
            let mut curr = prev_end.max(start);
            while curr < end {
                let mut rightmost = curr;
                while idx < n && available_intervals[idx][0] <= curr {
                    rightmost = rightmost.max(available_intervals[idx][1]);
                    idx += 1
                }
                if rightmost <= curr {
                    return -1;
                }
                curr = rightmost;
                res += 1;
            }
            prev_end = prev_end.max(curr);
        }
    }
    res
}

// 1 2 3 4
//     3 4

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
            min_selection(
                vec![[1, 4], [1, 7], [3, 9], [5, 10], [10, 15]],
                vec![[1, 3], [2, 5], [7, 8], [10, 13], [11, 15], [12, 13]]
            ),
            3
        );
    }

    #[test]
    fn test() {}
}
