mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn minimum_white_tiles(floor: &str, num_carpets: i32, carpet_len: i32) -> i32 {
    let n = floor.len();
    let prefix = floor.bytes().fold(Vec::with_capacity(n), |mut acc, b| {
        acc.push(i32::from(b - b'0') + acc.last().unwrap_or(&0));
        acc
    });
    prefix[n - 1]
        - dfs(
            floor.as_bytes(),
            &prefix,
            0,
            num_carpets as _,
            carpet_len as _,
            &mut vec![vec![-1; 1 + num_carpets as usize]; n],
        )
}

fn dfs(
    floor: &[u8],
    prefix: &[i32],
    idx: usize,
    num_carpets: usize,
    carpet_len: usize,
    memo: &mut [Vec<i32>],
) -> i32 {
    let n = floor.len();
    if idx >= n {
        return 0;
    }
    if num_carpets == 0 {
        return 0;
    }
    if memo[idx][num_carpets] > -1 {
        return memo[idx][num_carpets];
    }
    if floor[idx] == b'0' {
        return dfs(floor, prefix, 1 + idx, num_carpets, carpet_len, memo);
    }
    let skip = dfs(floor, prefix, 1 + idx, num_carpets, carpet_len, memo);
    let right = prefix.get(idx + carpet_len - 1).unwrap_or(&prefix[n - 1]);
    let del = right - if idx > 0 { prefix[idx] - 1 } else { 0 };
    let take = del
        + dfs(
            floor,
            prefix,
            idx + carpet_len,
            num_carpets - 1,
            carpet_len,
            memo,
        );
    memo[idx][num_carpets] = skip.max(take);
    memo[idx][num_carpets]
}

pub fn add_carpets(floor: &str, num_carpets: i32, carpet_len: i32) -> i32 {
    let n = floor.len();
    let [numc, len] = [num_carpets, carpet_len].map(|v| v as usize);
    if numc * len >= n {
        return 0;
    }
    let mut dp = floor.bytes().fold(Vec::with_capacity(n), |mut acc, b| {
        acc.push(i32::from(b - b'0') + acc.last().unwrap_or(&0));
        acc
    });
    for carp in 1..=numc {
        let mut next = vec![0; n];
        for idx in carp * len..n {
            next[idx] =
                (next[idx - 1] + i32::from(floor.as_bytes()[idx] - b'0')).min(dp[idx - len]);
        }
        dp = next;
    }
    dp[n - 1]
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
        assert_eq!(minimum_white_tiles("10110101", 2, 2), 2);
        assert_eq!(minimum_white_tiles("11111", 2, 3), 0);

        assert_eq!(add_carpets("10110101", 2, 2), 2);
        assert_eq!(add_carpets("11111", 2, 3), 0);
    }

    #[test]
    fn test() {}
}
