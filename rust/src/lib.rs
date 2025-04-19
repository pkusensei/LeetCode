mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_sequence(n: i32, sick: &[i32]) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let mut fact = vec![0; 1 + n as usize];
    fact[..2].fill(1);
    for v in 2..=n {
        fact[v as usize] = i64::from(v) * fact[v as usize - 1] % MOD;
    }
    let mut s = 0;
    let mut len_fact = 1;
    let mut k = 0;
    if sick[0] > 0 {
        let seg = i64::from(sick[0]); // left end
        s += seg;
        len_fact = (len_fact * fact[seg as usize]) % MOD;
    }
    for w in sick.windows(2) {
        if w[1] - w[0] > 1 {
            let seg = i64::from(w[1] - w[0] - 1);
            s += seg;
            len_fact = (len_fact * fact[seg as usize]) % MOD;
            k += seg - 1;
        }
    }
    if let Some(&v) = sick.last() {
        if n - v > 1 {
            let seg = i64::from(n - v - 1); // right end
            s += seg;
            len_fact = (len_fact * fact[seg as usize]) % MOD;
        }
    }
    let res = (fact[s as usize] * mod_pow(len_fact, MOD - 2, MOD)) % MOD * mod_pow(2, k, MOD) % MOD;
    res as i32
}

const fn mod_pow(base: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    };
    if exp & 1 == 0 {
        mod_pow(base * base % m, exp >> 1, m)
    } else {
        base * mod_pow(base * base % m, exp >> 1, m) % m
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
        assert_eq!(number_of_sequence(5, &[0, 4]), 4);
        assert_eq!(number_of_sequence(4, &[1]), 3);
    }

    #[test]
    fn test() {}
}
