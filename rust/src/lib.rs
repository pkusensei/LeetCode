mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn min_days(n: i32) -> i32 {
    let mut memo = vec![-1; 1 + n as usize];
    dfs(n, &mut memo)
}

fn dfs(n: i32, memo: &mut [i32]) -> i32 {
    if let Ok(i) = SUMS.binary_search(&n) {
        return i as i32;
    }
    if memo[n as usize] > -1 {
        return memo[n as usize];
    }
    let i = SUMS.partition_point(|&v| v < n);
    let mut res = i32::MAX;
    for d in 1..i {
        let delta = n - d as i32 * (1 + d as i32) / 2;
        res = res.min(d as i32 + 1 + dfs(delta, memo));
    }
    memo[n as usize] = res;
    res
}

// 447*448/2 = 100_128
const SUMS: [i32; 448] = f();
const fn f() -> [i32; 448] {
    let mut res = [0; 448];
    let mut i = 1;
    while i <= 447 {
        res[i] = i as i32 * (1 + i as i32) / 2;
        i += 1;
    }
    res
}

// n(1+n)/2

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
        assert_eq!(min_days(2), 3);
    }

    #[test]
    fn test() {
        assert_eq!(min_days(7), 5);
    }
}
