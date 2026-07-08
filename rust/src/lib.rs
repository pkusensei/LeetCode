mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_and_multiply(s: &str, queries: &[[i32; 2]]) -> Vec<i32> {
    let mut pref_x = vec![];
    let mut pref_pow = vec![];
    let mut pref_sum = vec![];
    let mut sum = 0;
    let mut x = 0;
    let mut pow = 0;
    for b in s.bytes() {
        let d = i64::from(b - b'0');
        if d > 0 {
            sum = (sum + d) % M;
            x = (x * 10 % M + d) % M;
            pow += 1;
        }
        pref_x.push(x);
        pref_sum.push(sum);
        pref_pow.push(pow);
    }
    let mut res = vec![];
    for q in queries {
        let [left, right] = [0, 1].map(|v| q[v] as usize);
        if let Some(left) = left.checked_sub(1) {
            let sum = (pref_sum[right] - pref_sum[left]).rem_euclid(M);
            let pow = pref_pow[right] - pref_pow[left];
            let x = pref_x[right] - pref_x[left] * mod_pow(10, pow) % M;
            res.push((x.rem_euclid(M) * sum % M) as i32);
        } else {
            res.push((pref_x[right] * pref_sum[right] % M) as i32);
        }
    }
    res
}

const M: i64 = 1_000_000_007;

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
        assert_eq!(
            sum_and_multiply("10203004", &[[0, 7], [1, 3], [4, 6]]),
            [12340, 4, 9]
        );
    }

    #[test]
    fn test() {}
}
