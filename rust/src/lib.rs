mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_weight(intervals: Vec<Vec<i32>>) -> Vec<i32> {
    let n = intervals.len();
    // idx, left, right, weight
    let mut arr: Vec<_> = intervals
        .iter()
        .enumerate()
        .map(|(i, interval)| (i, interval[0], interval[1], i64::from(interval[2])))
        .collect();
    arr.sort_unstable_by(|a, b| a.2.cmp(&b.2).then_with(|| a.1.cmp(&b.1)));
    let mut dp_w = vec![[0; 5]; 1 + n];
    let mut dp_idx = vec![vec![Vec::<i32>::new(); 5]; 1 + n];
    for (arr_i, &(int_i, left, _right, weight)) in arr.iter().enumerate() {
        // forces dp array to be of length `1+n`
        // because prev=0 does not guarantee `0` is valid option
        // `1+n` turns it into always valid in dp array
        let prev = arr[..arr_i].partition_point(|v| v.2 < left);
        for len in 1..=4 {
            let skip = dp_w[arr_i][len];
            let take = dp_w[prev][len - 1] + weight;
            if skip > take {
                dp_w[1 + arr_i][len] = skip;
                dp_idx[1 + arr_i][len] = dp_idx[arr_i][len].clone();
            } else {
                dp_w[1 + arr_i][len] = take;
                let mut curr = dp_idx[prev][len - 1].clone();
                curr.push(int_i as i32);
                curr.sort_unstable();
                let skip_idx = &dp_idx[arr_i][len];
                if skip == take && skip_idx < &curr {
                    curr = dp_idx[arr_i][len].clone();
                }
                dp_idx[1 + arr_i][len] = curr;
            }
        }
    }
    dp_idx[n][4].clone()
}

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
    fn basics() {}

    #[test]
    fn test() {}
}
