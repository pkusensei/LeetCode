mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
    let mut memo =
        vec![vec![vec![vec![-1; 1 + limit as usize]; 3]; 1 + one as usize]; 1 + zero as usize];
    dfs(zero, one, limit, 2, 0, &mut memo)
}

fn dfs(
    num_zero: i32,
    num_one: i32,
    limit: i32,
    prev: i32,
    count: i32,
    memo: &mut [Vec<Vec<Vec<i32>>>],
) -> i32 {
    const M: i32 = 1_000_000_007;
    if num_zero == 0 && num_one == 0 {
        return i32::from(count <= limit);
    }
    if num_zero < 0 || num_one < 0 || count > limit {
        return 0;
    }
    if memo[num_zero as usize][num_one as usize][prev as usize][count as usize] > -1 {
        return memo[num_zero as usize][num_one as usize][prev as usize][count as usize];
    }
    let res = dfs(
        num_zero - 1,
        num_one,
        limit,
        0,
        if prev == 0 { 1 + count } else { 1 },
        memo,
    ) + dfs(
        num_zero,
        num_one - 1,
        limit,
        1,
        if prev == 1 { 1 + count } else { 1 },
        memo,
    );
    memo[num_zero as usize][num_one as usize][prev as usize][count as usize] = res % M;
    memo[num_zero as usize][num_one as usize][prev as usize][count as usize]
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
        assert_eq!(number_of_stable_arrays(1, 1, 2), 2);
        assert_eq!(number_of_stable_arrays(1, 2, 1), 1);
        assert_eq!(number_of_stable_arrays(3, 3, 2), 14);
    }

    #[test]
    fn test() {}
}
