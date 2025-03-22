mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn beautiful_partitions(s: &str, k: i32, min_length: i32) -> i32 {
    let (s, n) = (s.as_bytes(), s.len());
    if !P.contains(&s[0]) || P.contains(&s[n - 1]) {
        return 0;
    }
    let [k, len] = [k, min_length].map(|v| v as usize);
    dfs(s, k - 1, len, len, &mut vec![vec![-1; n]; k])
}

const P: &[u8] = b"2357";

fn dfs(s: &[u8], k: usize, len: usize, idx: usize, memo: &mut [Vec<i32>]) -> i32 {
    let n = s.len();
    if k == 0 {
        return i32::from(idx <= n);
    }
    if idx >= n {
        return 0;
    }
    if memo[k][idx] > -1 {
        return memo[k][idx];
    }
    // skip
    let mut res = dfs(s, k, len, 1 + idx, memo);
    // take
    if P.contains(&s[idx]) && !P.contains(&s[idx - 1]) {
        res += dfs(s, k - 1, len, idx + len, memo);
    }
    res %= 1_000_000_007;
    memo[k][idx] = res;
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
        assert_eq!(beautiful_partitions("23542185131", 3, 2), 3);
        assert_eq!(beautiful_partitions("23542185131", 3, 3), 1);
        assert_eq!(beautiful_partitions("3312958", 3, 1), 1);
    }

    #[test]
    fn test() {}
}
