mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn with_binary_search(stone_value: &[i32]) -> i32 {
    let n = stone_value.len();
    // len(prefix) = 1+n
    let prefix = stone_value.iter().fold(vec![0], |mut acc, v| {
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
            let sum = prefix[1 + right] - prefix[left];
            let i = search(&prefix, left, right);
            // let i = prefix.partition_point(|v| v - prefix[left] < prefix[1 + right] - v) - 1;
            let left_half = prefix[1 + i] - prefix[left];
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
