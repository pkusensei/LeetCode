mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_permutations(n: i32, requirements: &[[i32; 2]]) -> i32 {
    const M: i64 = 1_000_000_007;
    let n = n as usize;
    let mut max_end = 0;
    let mut max_cnt = 0;
    let mut req = vec![None; n];
    for r in requirements.iter() {
        let [end, cnt] = r[..] else { unreachable!() };
        if end == 0 && cnt > 0 {
            return 0;
        }
        req[end as usize] = Some(cnt as usize);
        max_end = max_end.max(end as usize);
        max_cnt = max_cnt.max(cnt as usize);
    }
    let mut dp = vec![0; 1 + max_cnt];
    dp[0] = 1;
    for idx in 0..=max_end {
        let mut curr = vec![0; 1 + max_cnt];
        if let Some(end) = req[idx] {
            curr[end] = dp[end.saturating_sub(idx)..=end]
                .iter()
                .fold(0, |acc, v| (acc + v) % M);
        } else {
            for c in 0..=max_cnt {
                curr[c] = dp[c];
                if let Some(prev) = c.checked_sub(1) {
                    curr[c] = (curr[c] + curr[prev]) % M;
                }
                if let Some(prev) = c.checked_sub(1 + idx) {
                    curr[c] = (curr[c] - dp[prev]).rem_euclid(M);
                }
            }
        }
        dp = curr;
    }
    let mut res = dp.iter().fold(0, |acc, v| (acc + v) % M);
    for i in 1 + max_end..n {
        res = res * (1 + i) as i64 % M;
    }
    res as i32
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
        assert_eq!(number_of_permutations(3, &[[2, 2], [0, 0]]), 2);
        assert_eq!(number_of_permutations(3, &[[2, 2], [1, 1], [0, 0]]), 1);
        assert_eq!(number_of_permutations(2, &[[0, 0], [1, 0]]), 1);
    }

    #[test]
    fn test() {}
}
