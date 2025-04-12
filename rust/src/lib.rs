mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn number_of_ways(s: &str, t: &str, k: i64) -> i32 {
    const MOD: i64 = 1_000_000_007;
    let n = t.len();
    let mut s = s.as_bytes().to_vec();
    s.extend_from_within(..n - 1);
    let pos = kmp(&s, t.as_bytes());
    let mut f_k = [0, 0];
    f_k[1] = (mod_pow(n as i64 - 1, k, MOD) + ((k & 1) * 2 - 1)).rem_euclid(MOD)
        * mod_pow(n as i64, MOD - 2, MOD);
    f_k[0] = (f_k[1] - ((k & 1) * 2 - 1)).rem_euclid(MOD);
    let mut res = 0;
    for p in pos {
        if p == 0 {
            res += f_k[0]
        } else {
            res += f_k[1]
        }
        res %= MOD;
    }
    res as i32
}

fn kmp(s: &[u8], t: &[u8]) -> Vec<usize> {
    let n = t.len();
    let mut f = vec![0; n];
    for i1 in 1..n {
        let mut i2 = f[i1 - 1];
        while i2 > 0 && t[i2] != t[i1] {
            i2 = f[i2 - 1];
        }
        if i2 == 0 && t[0] != t[i1] {
            f[i1] = 0;
        } else {
            f[i1] = 1 + i2;
        }
    }
    let mut i2 = 0;
    let mut res = vec![];
    for (i1, &byte) in s.iter().enumerate() {
        while i2 >= n || (i2 > 0 && byte != t[i2]) {
            i2 = f[i2 - 1];
        }
        if byte == t[i2] {
            i2 += 1
        }
        if i2 == n {
            res.push(i1 + 1 - n);
        }
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
        assert_eq!(number_of_ways("abcd", "cdab", 2), 2);
        assert_eq!(number_of_ways("ababab", "ababab", 1), 2);
    }

    #[test]
    fn test() {}
}
