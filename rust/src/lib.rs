mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_ways(n: i32, x: i32, y: i32) -> i32 {
    let mut res = 0;
    let [comb, dp] = &MATS;
    for st in 1..=n.min(x) as usize {
        // Fix stage number st
        // Each stage can be assigned 1..=y, i.e pow(y, st)
        // To pick st out of x =>  comb[x][st]
        // Put all n onto st stages => dp[n][st]
        res += mod_pow(y as usize, st, MOD) * comb[x as usize][st] % MOD
            * dp[n as usize][st as usize]
            % MOD;
        res %= MOD;
    }
    res as i32
}

const N: usize = 1001;
const MOD: usize = 1_000_000_007;
const MATS: [[[usize; N]; N]; 2] = precompute();

const fn precompute() -> [[[usize; N]; N]; 2] {
    let [mut comb, mut dp] = [[[0; N]; N]; 2];
    comb[0][0] = 1; // comb[all_rooms][picked_rooms]
    dp[0][0] = 1; // dp[performer][room]
    let mut row = 1;
    while row < N {
        comb[row][0] = 1;
        let mut col = 1;
        while col <= row {
            // Pascal triangle
            comb[row][col] = (comb[row - 1][col - 1] + comb[row - 1][col]) % MOD;
            // 1) Start with one less performer and same rooms, dp[row-1][col]
            //    The new performer can join any room.
            // 2) One less performer and one less room, dp[row-1][col-1]
            //    New performer can start any room, and they are labelled.
            dp[row][col] = col * (dp[row - 1][col - 1] + dp[row - 1][col]) % MOD;
            col += 1;
        }
        row += 1;
    }
    [comb, dp]
}

const fn mod_pow(b: usize, exp: usize, m: usize) -> usize {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(b * b % m, exp >> 1, m)
    } else {
        mod_pow(b * b % m, exp >> 1, m) * b % m
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
        assert_eq!(number_of_ways(1, 2, 3), 6);
        assert_eq!(number_of_ways(5, 2, 1), 32);
        assert_eq!(number_of_ways(3, 3, 4), 684);
    }

    #[test]
    fn test() {}
}
