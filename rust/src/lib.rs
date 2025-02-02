mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn ways_to_fill_array(queries: &[[i32; 2]]) -> Vec<i32> {
    const PRIMES: [i32; 25] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    let mut memo = HashMap::new();
    let mut res = vec![];
    for q in queries.iter() {
        let mut count = 1;
        let [n, mut k] = [q[0], q[1]];
        for prime in PRIMES {
            let mut curr = 0;
            while k % prime == 0 {
                k /= prime;
                curr += 1;
            }
            count *= n_choose_k(n - 1 + curr, curr, &mut memo);
            count %= MOD;
        }
        if k != 1 {
            count *= i64::from(n);
            count %= MOD;
        }
        res.push(count as i32);
    }
    res
}

const MOD: i64 = 1_000_000_007;

fn n_choose_k(n: i32, k: i32, memo: &mut HashMap<[i32; 2], i64>) -> i64 {
    if let Some(&v) = memo.get(&[n, k]) {
        return v;
    }
    let res = if k > n {
        0
    } else if k == 0 || k == n {
        1
    } else {
        (n_choose_k(n - 1, k - 1, memo) + n_choose_k(n - 1, k, memo)) % MOD
    };
    memo.insert([n, k], res);
    res
}

#[cfg(test)]
mod tests {

    use super::*;

    #[allow(unused_macros)]
    macro_rules! sort_eq {
        ($a:expr, $b:expr) => {
            let (mut left, mut right) = ($a, $b);
            left.sort_unstable();
            right.sort_unstable();
            assert_eq!(left, right);
        };
    }

    const _EP: f64 = 1e-5;
    #[allow(unused_macros)]
    macro_rules! float_eq {
        ($a:expr, $b:expr) => {
            let (left, right) = ($a, $b);
            assert!(
                (left - right).abs() <= _EP,
                "left = {:?}, right = {:?}",
                left,
                right
            );
        };
    }

    #[test]
    fn basics() {
        assert_eq!(
            ways_to_fill_array(&[[2, 6], [5, 1], [73, 660]]),
            [4, 1, 50734910]
        );
        assert_eq!(
            ways_to_fill_array(&[[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]]),
            [1, 2, 3, 10, 5]
        );
    }

    #[test]
    fn test() {}
}
