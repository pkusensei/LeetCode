mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn count_anagrams(s: &str) -> i32 {
    let f = facts(100_001);
    s.split_ascii_whitespace()
        .map(|w| {
            let count = w.bytes().fold([0; 26], |mut acc, b| {
                acc[usize::from(b - b'a')] += 1;
                acc
            });
            let mut v = f[w.len()];
            for c in count {
                if c > 0 {
                    v = v * mod_pow(f[c], MOD - 2, MOD) % MOD;
                }
            }
            v
        })
        .fold(1, |acc, v| acc * v % MOD) as i32
}

// For big number x, y
// to find (x/y)%mod
// use x*mod_pow(y, mod-2, mod)

const MOD: i64 = 1_000_000_007;

fn facts(n: usize) -> Vec<i64> {
    let mut res = vec![1; n];
    for i in 2..n {
        res[i] = res[i - 1] * i as i64 % MOD;
    }
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
        assert_eq!(count_anagrams("too hot"), 18);
        assert_eq!(count_anagrams("aa"), 1);
    }

    #[test]
    fn test() {}
}
