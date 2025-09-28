mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
    let [n, low, high] = [n, l - l, r - l].map(|v| v as usize);
    let width = high + 1 - low;
    let mut dp_inc = vec![1; width];
    let mut dp_dec = vec![1; width];
    for _ in 1..n {
        let mut curr_inc = vec![0; width];
        let mut curr_dec = vec![0; width];
        for i in 1 + low..=high {
            curr_inc[i] = (dp_dec[i - 1] + curr_inc[i - 1]) % M;
        }
        for i in (low..high).rev() {
            curr_dec[i] = (dp_inc[1 + i] + curr_dec[1 + i]) % M;
        }
        dp_inc = curr_inc;
        dp_dec = curr_dec;
    }
    dp_inc
        .into_iter()
        .chain(dp_dec)
        .fold(0, |acc, v| (acc + v) % M)
    // let [n, left, right] = [n, l, r].map(|v| v as usize);
    // let width = right - left + 1;
    // let mut memo = vec![vec![vec![-1; width]; 2]; n];
    // (left..=right)
    //     .map(|curr| {
    //         (dfs(n, left, right, 0, curr, &mut memo) + dfs(n, left, right, 1, curr, &mut memo)) % M
    //     })
    //     .fold(0, |acc, v| (acc + v) % M)
}

const M: i32 = 1_000_000_007;

// tle's
fn dfs(
    n: usize,
    left: usize,
    right: usize,
    dir: usize,
    curr: usize,
    memo: &mut [Vec<Vec<i32>>],
) -> i32 {
    if n == 1 {
        return 1;
    }
    if memo[n - 1][dir][curr - left] > -1 {
        return memo[n - 1][dir][curr - left];
    }
    let range = if dir == 1 {
        left..curr
    } else {
        1 + curr..1 + right
    };
    let res = range
        .map(|val| dfs(n - 1, left, right, 1 ^ dir, val, memo))
        .fold(0, |acc, v| (acc + v) % M);
    memo[n - 1][dir][curr - left] = res;
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
        assert_eq!(zig_zag_arrays(3, 1, 3), 10);
        assert_eq!(zig_zag_arrays(3, 4, 5), 2);
    }

    #[test]
    fn test() {}
}
