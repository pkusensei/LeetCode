mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_good_subsets(nums: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    const PRIMES: [i32; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut dp = vec![0; 1 + (1 << 10)]; // total of 10 primes
    dp[0] = 1;
    let count = nums.iter().fold([0; 31], |mut acc, &num| {
        acc[num as usize] += 1;
        acc
    });
    for num in 1..=30 {
        if count[num as usize] == 0 {
            continue; // absent
        }
        if num == 1 {
            continue; // special case
        }
        if [4, 9, 25].iter().any(|v| num % v == 0) {
            // 4,9,25 - dupe primes
            continue;
        }
        // Prime factors of this number
        let mask: usize = PRIMES
            .iter()
            .enumerate()
            .filter_map(|(i, &p)| if num % p == 0 { Some(1 << i) } else { None })
            .sum(); // same as fold acc|(1<<i)
        for subset in 0..1 << 10 {
            if subset & mask > 0 {
                continue; // intersects => dupe
            }
            let new_mask = subset | mask; // Add num into subset
            dp[new_mask] += dp[subset] * count[num as usize];
            dp[new_mask] %= MOD;
        }
    }
    // [1..] to exclude empty subset
    // 1's can be freely added
    let res = dp[1..].iter().fold(0, |acc, v| (acc + v) % MOD) * mod_pow(2, count[1], MOD);
    (res % MOD) as _
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
        assert_eq!(number_of_good_subsets(&[1, 2, 3, 4]), 6);
        assert_eq!(number_of_good_subsets(&[4, 2, 3, 15]), 5);
    }

    #[test]
    fn test() {
        assert_eq!(number_of_good_subsets(&[12, 3]), 1);
    }
}
