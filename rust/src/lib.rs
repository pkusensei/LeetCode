mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn make_string_sorted(s: &str) -> i32 {
    const MOD: i64 = 1_000_000_007;

    let n = s.len();
    let mut facts = Vec::with_capacity(1 + n);
    facts.push(1);
    for i in 1..=n {
        facts.push(i as i64 * facts.last().unwrap_or(&1) % MOD);
    }
    let mut res = 0;
    let mut freqs = [0; 26];
    for (idx, b) in s.bytes().enumerate().rev() {
        let pos = usize::from(b - b'a');
        freqs[pos] += 1;
        let mut curr = freqs[..pos].iter().sum::<i64>() * facts[n - idx - 1] % MOD;
        for &freq in freqs.iter() {
            let fact = facts[freq as usize];
            curr *= mod_pow(fact, MOD - 2, MOD);
            curr %= MOD;
        }
        res += curr;
    }
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
        assert_eq!(make_string_sorted("cba".into()), 5);
        assert_eq!(make_string_sorted("aabaa".into()), 2);
    }

    #[test]
    fn test() {
        assert_eq!(
            make_string_sorted(
                "fbefskzvhfdclkwavtmejwmxavhrhidpiwdjjyrxqvjjkalqqjbmklwlmhjmuzrlbsyn"
            ),
            857325869
        );
    }
}
