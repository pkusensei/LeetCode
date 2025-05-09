mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_balanced_permutations(num: &str) -> i32 {
    let mut freq = [0; 10];
    let mut sum = 0;
    for b in num.bytes() {
        freq[usize::from(b - b'0')] += 1;
        sum += i32::from(b - b'0');
    }
    if sum & 1 == 1 {
        return 0;
    }
    let n = num.len();
    let mut memo = vec![vec![vec![-1; 1 + sum as usize / 2]; 1 + n / 2]; 1 + n];
    let mut res = dfs(num.as_bytes(), 0, n / 2, sum / 2, &mut memo);
    let (fact, invf) = fact_inv(n);
    res = res * fact[n / 2] % M * fact[n - n / 2] % M;
    for f in freq {
        res = res * invf[f] % M
    }
    res as i32
}

const M: i64 = 1_000_000_007;

fn fact_inv(n: usize) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![1];
    for i in 1..=n {
        fact.push(fact[i - 1] * i as i64 % M);
    }
    let mut invf = vec![0; 1 + n];
    invf[n] = mod_pow(fact[n], M - 2, M);
    for i in (0..n).rev() {
        invf[i] = invf[1 + i] * (1 + i) as i64 % M;
    }
    (fact, invf)
}

fn dfs(s: &[u8], idx: usize, count: usize, sum: i32, memo: &mut [Vec<Vec<i64>>]) -> i64 {
    if count == 0 {
        return i64::from(sum == 0);
    }
    if idx >= s.len() || sum < 0 {
        return 0;
    }
    if memo[idx][count][sum as usize] > -1 {
        return memo[idx][count][sum as usize];
    }
    let res = (dfs(s, 1 + idx, count, sum, memo)
        + dfs(s, 1 + idx, count - 1, sum - i32::from(s[idx] - b'0'), memo))
        % M;
    memo[idx][count][sum as usize] = res;
    res
}

const fn mod_pow(b: i64, e: i64, m: i64) -> i64 {
    if e == 0 {
        return 1;
    }
    if e & 1 == 0 {
        mod_pow(b * b % m, e >> 1, m)
    } else {
        mod_pow(b * b % m, e >> 1, m) * b % m
    }
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
        assert_eq!(count_balanced_permutations("123"), 2);
        assert_eq!(count_balanced_permutations("112"), 1);
        assert_eq!(count_balanced_permutations("12345"), 0);
    }

    #[test]
    fn test() {
        assert_eq!(count_balanced_permutations("4567"), 8);
    }
}
