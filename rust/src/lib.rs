mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn maximum_coins(mut coins: Vec<[i32; 3]>, k: i32) -> i64 {
    let n = coins.len();
    coins.sort_unstable_by_key(|c| c[0]);
    let mut starts = Vec::with_capacity(n);
    let mut ends = Vec::with_capacity(n);
    let mut prefix = vec![0];
    for c in coins.iter() {
        let [left, right, v] = c[..] else {
            unreachable!()
        };
        starts.push(left);
        ends.push(right);
        let val = i64::from(right + 1 - left) * i64::from(v);
        prefix.push(val + prefix.last().unwrap_or(&0));
    }
    let mut res = 0;
    // Left-aligned window [left, left + k - 1]
    for (idx, &left) in starts.iter().enumerate() {
        let target = left + k - 1;
        let i = ends.partition_point(|&v| v < target);
        let mut temp = prefix[i] - prefix[idx];
        if i < n {
            let overlap = (target - coins[i][0] + 1).max(0);
            temp += i64::from(overlap) * i64::from(coins[i][2]);
        }
        res = res.max(temp);
    }
    // Right-aligned window [right - k + 1, right]
    for (idx, &right) in ends.iter().enumerate() {
        let target = right - k + 1;
        let i = starts.partition_point(|&v| v < target);
        let mut curr = prefix[idx + 1] - prefix[i];
        if i > 0 {
            let overlap = (coins[i - 1][1] - target + 1).max(0);
            curr += i64::from(overlap) * i64::from(coins[i - 1][2]);
        }
        res = res.max(curr);
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
        assert_eq!(maximum_coins(vec![[8, 10, 1], [1, 3, 2], [5, 6, 4]], 4), 10);
        assert_eq!(maximum_coins(vec![[1, 10, 3]], 2), 6);
    }

    #[test]
    fn test() {}
}
