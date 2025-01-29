mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn box_delivering(
    boxes: &[[i32; 2]],
    _ports_count: i32,
    mut max_boxes: i32,
    mut max_weight: i32,
) -> i32 {
    let n = boxes.len();
    let [mut right, mut last_right] = [0, 0];
    let mut need = 0; // trips needed to do [left..=right]
    let mut dp = vec![200_000; 1 + n];
    dp[0] = 0;
    for (left, b) in boxes.iter().enumerate() {
        while right < n && max_boxes > 0 && max_weight >= boxes[right][1] {
            max_boxes -= 1;
            max_weight -= boxes[right][1];
            if right == 0 || boxes[right][0] != boxes[right - 1][0] {
                last_right = right; // [last_right..=right] of same port
                need += 1; // new port adds one trip
            }
            right += 1;
        }
        // take as many as possible
        dp[right] = dp[right].min(dp[left] + 1 + need);
        // save one trip/need, but carry less weight
        dp[last_right] = dp[last_right].min(dp[left] + need);
        // move left ptr forward; this box was shipped during previous trip
        max_boxes += 1;
        max_weight += b[1];
        // this box is of a different port
        if left == n - 1 || b[0] != boxes[left + 1][0] {
            need -= 1;
        }
    }
    dp[n]

    // let prefix = boxes.iter().fold(Vec::with_capacity(n), |mut acc, b| {
    //     acc.push(b[1] + acc.last().unwrap_or(&0));
    //     acc
    // });
    // dfs(
    //     boxes,
    //     &prefix,
    //     max_boxes as _,
    //     max_weight,
    //     0,
    //     &mut vec![-1; n],
    // )
}

// TLEs
fn dfs(
    boxes: &[[i32; 2]],
    prefix: &[i32],
    max_boxes: usize,
    max_weight: i32,
    idx: usize,
    memo: &mut [i32],
) -> i32 {
    let n = boxes.len();
    if idx >= n {
        return 0;
    }
    if memo[idx] > -1 {
        return memo[idx];
    }
    let mut res = i32::MAX;
    let mut port_count = 1;
    let mut curr_port = boxes[idx][0];
    let mut end = 0;
    for (i, b) in boxes.iter().enumerate().skip(idx).take(max_boxes) {
        let curr_weight = prefix[i] - if idx > 0 { prefix[idx - 1] } else { 0 };
        if curr_weight > max_weight {
            break;
        }
        if b[0] != curr_port {
            // new port
            curr_port = b[0];
            res = res.min(2 + port_count - 1 + dfs(boxes, prefix, max_boxes, max_weight, i, memo));
            port_count += 1;
        }
        end = i;
    }
    // take as many as possible
    res = res.min(2 + port_count - 1 + dfs(boxes, prefix, max_boxes, max_weight, 1 + end, memo));
    memo[idx] = res;
    res
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    #[test]
    fn basics() {
        assert_eq!(box_delivering(&[[1, 1], [2, 1], [1, 1]], 2, 3, 3), 4);
        assert_eq!(
            box_delivering(&[[1, 2], [3, 3], [3, 1], [3, 1], [2, 4]], 3, 3, 6),
            6
        );
        assert_eq!(
            box_delivering(&[[1, 4], [1, 2], [2, 1], [2, 1], [3, 2], [3, 4]], 3, 6, 7),
            6
        );
    }

    #[test]
    fn test() {}

    #[allow(dead_code)]
    fn float_eq<T1, T2>(a: T1, b: T2)
    where
        T1: Into<f64> + Copy + Debug,
        T2: Into<f64> + Copy + Debug,
    {
        const EP: f64 = 1e-5;
        assert!(
            (<T1 as Into<f64>>::into(a) - <T2 as Into<f64>>::into(b)).abs() <= EP,
            "left = {a:?}, right = {b:?}"
        );
    }
}
