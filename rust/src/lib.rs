mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn magical_sum(m: i32, k: i32, nums: &[i32]) -> i32 {
    let m = m as usize;
    let comb = n_choose_k(m, m);
    let mut memo = vec![vec![vec![vec![-1; 1 + m]; nums.len()]; 1 + k as usize]; 1 + m];
    dfs(nums, &comb, m, k, 0, 0, &mut memo) as i32
}

const M: i64 = 1_000_000_007;

fn dfs(
    nums: &[i32],
    comb: &[Vec<i64>],
    m: usize,
    k: i32,
    idx: usize,
    carry: i32,
    memo: &mut [Vec<Vec<Vec<i64>>>],
) -> i64 {
    if k < 0 || (m + carry.count_ones() as usize) < k as usize {
        return 0;
    }
    if m == 0 {
        return i64::from(carry.count_ones() as i32 == k);
    }
    if idx >= nums.len() {
        return 0;
    }
    if memo[m][k as usize][idx][carry as usize] > -1 {
        return memo[m][k as usize][idx][carry as usize];
    }
    let mut res = 0;
    for count in 0..=m {
        let c = comb[m][count] * mod_pow(nums[idx].into(), count as _) % M;
        let ncarry = carry + count as i32;
        res += c * dfs(
            nums,
            comb,
            m - count,
            k - (ncarry & 1),
            1 + idx,
            ncarry >> 1,
            memo,
        );
        res %= M;
    }
    memo[m][k as usize][idx][carry as usize] = res;
    res
}

fn n_choose_k(n: usize, k: usize) -> Vec<Vec<i64>> {
    let mut res = vec![vec![0; 1 + k]; 1 + n];
    res[0][0] = 1;
    for row in 1..=n {
        res[row][0] = 1;
        for col in 1..=row {
            res[row][col] = (res[row - 1][col - 1] + res[row - 1][col]) % M;
        }
    }
    res
}

const fn mod_pow(b: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % M, exp >> 1)
    } else {
        mod_pow(b * b % M, exp >> 1) * b % M
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
        assert_eq!(magical_sum(5, 5, &[1, 10, 100, 10000, 1000000]), 991600007);
        assert_eq!(magical_sum(2, 2, &[5, 4, 3, 2, 1]), 170);
    }

    #[test]
    fn test() {}
}
