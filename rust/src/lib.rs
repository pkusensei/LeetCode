mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn final_opt(stone_value: &[i32]) -> i32 {
    let n = stone_value.len();
    let mut dp = vec![vec![0; n]; n];
    let mut max_left = dp.clone();
    let mut max_right = dp.clone();
    for (left, &val) in stone_value.iter().enumerate().rev() {
        max_left[left][left] = val;
        max_right[left][left] = val;
        let mut total = val;
        let mut sum_left = 0;
        let mut i = left as i32 - 1;
        for right in (1 + left)..n {
            total += stone_value[right];
            while 1 + i < right as i32 && (sum_left + stone_value[(1 + i) as usize]) * 2 <= total {
                sum_left += stone_value[(1 + i) as usize];
                i += 1;
            }
            if left as i32 <= i {
                dp[left][right] = dp[left][right].max(max_left[left][i as usize]);
            }
            if 1 + i < right as i32 {
                dp[left][right] = dp[left][right].max(max_right[(2 + i) as usize][right]);
            }
            if 2 * sum_left == total {
                dp[left][right] = dp[left][right].max(max_right[(1 + i) as usize][right]);
            }
            max_left[left][right] = max_left[left][right - 1].max(total + dp[left][right]);
            max_right[left][right] = max_right[1 + left][right].max(total + dp[left][right]);
        }
    }
    dp[0][n - 1]
}

pub fn with_binary_search(stone_value: &[i32]) -> i32 {
    let n = stone_value.len();
    let prefix = stone_value.iter().fold(vec![], |mut acc, v| {
        acc.push(v + acc.last().unwrap_or(&0));
        acc
    });
    let mut dp = vec![vec![0; n]; n];
    let mut max_left = dp.clone();
    let mut max_right = dp.clone();
    for (i, &v) in stone_value.iter().enumerate() {
        max_left[i][i] = v;
        max_right[i][i] = v;
    }
    for len in 2..=n {
        for left in 0..=n - len {
            let right = left + len - 1;
            let drop_left = if left > 0 { prefix[left - 1] } else { 0 };
            let sum = prefix[right] - drop_left;
            // let i = search(&prefix, left, right);
            let i = prefix.partition_point(|v| v - drop_left < prefix[right] - v);
            let left_half = prefix[i] - drop_left;
            if left_half * 2 == sum {
                dp[left][right] = max_left[left][i].max(max_right[1 + i][right]);
            } else {
                let a = if i == left { 0 } else { max_left[left][i - 1] };
                let b = if i == right {
                    0
                } else {
                    max_right[1 + i][right]
                };
                dp[left][right] = a.max(b);
            }
            max_left[left][right] = max_left[left][right - 1].max(sum + dp[left][right]);
            max_right[left][right] = max_right[1 + left][right].max(sum + dp[left][right]);
        }
    }
    dp[0][n - 1]
}

fn search(prefix: &[i32], left: usize, right: usize) -> usize {
    let sum = prefix[1 + right] - prefix[left];
    let mut lo = left;
    let mut hi = right;
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if (prefix[1 + mid] - prefix[left]) * 2 >= sum {
            hi = mid
        } else {
            lo = 1 + mid
        }
    }
    lo
}

pub fn stone_game_v(stone_value: Vec<i32>) -> i32 {
    let n = stone_value.len();
    let prefix = stone_value.iter().fold(vec![], |mut acc, &v| {
        acc.push(v + acc.last().unwrap_or(&0));
        acc
    });
    let mut dp = vec![vec![0; n]; n];
    for len in 2..=n {
        for left in 0..=n - len {
            let right = left + len - 1;
            for i in left..right {
                let a = prefix[i] - if left > 0 { prefix[left - 1] } else { 0 };
                let b = prefix[right] - prefix[i];
                let curr = match a.cmp(&b) {
                    std::cmp::Ordering::Less => a + dp[left][i],
                    std::cmp::Ordering::Equal => a + dp[left][i].max(dp[1 + i][right]),
                    std::cmp::Ordering::Greater => b + dp[1 + i][right],
                };
                dp[left][right] = dp[left][right].max(curr);
            }
        }
    }
    dp[0][n - 1]
    // dfs(&prefix, 0, n - 1)
}

fn dfs(prefix: &[i32], left: usize, right: usize) -> i32 {
    if left >= right {
        return 0;
    }
    let mut res = 0;
    for i in left..right {
        let a = prefix[i] - if left > 0 { prefix[left - 1] } else { 0 };
        let b = prefix[right] - prefix[i];
        match a.cmp(&b) {
            std::cmp::Ordering::Less => {
                let curr = a + dfs(prefix, left, i);
                res = res.max(curr);
            }
            std::cmp::Ordering::Equal => {
                let curr = dfs(prefix, left, i).max(dfs(prefix, 1 + i, right));
                res = res.max(a + curr);
            }
            std::cmp::Ordering::Greater => {
                let curr = b + dfs(prefix, 1 + i, right);
                res = res.max(curr);
            }
        }
    }
    res
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
