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
    let n = s.len();
    let mut pref_sum = Vec::with_capacity(n);
    let mut pref_x = Vec::with_capacity(n);
    let mut pref_pow = Vec::with_capacity(n);
    let mut curr_val = 0;
    for b in s.bytes() {
        let d = i64::from(b - b'0');
        pref_sum.push(d + pref_sum.last().unwrap_or(&0));
        pref_pow.push(i64::from(d > 0) + pref_pow.last().unwrap_or(&0));
        curr_val *= if d == 0 { 1 } else { 10 };
        curr_val %= M;
        curr_val += d;
        curr_val %= M;
        pref_x.push(curr_val);
    }
    let mut res = vec![];
    for q in queries {
        let [left, right] = [0, 1].map(|i| q[i] as usize);
        let dsum = pref_sum[right] - if left > 0 { pref_sum[left - 1] } else { 0 };
        if dsum == 0 {
            res.push(0);
            continue;
        }
        let x = pref_x[right]
            - if left > 0 {
                pref_x[left - 1] * mod_pow(10, pref_pow[right] - pref_pow[left - 1], M) % M
            } else {
                0
            };
        res.push((x.rem_euclid(M) * dsum % M) as i32);
    }
    res
}

const M: i64 = 1_000_000_007;

const fn mod_pow(b: i64, exp: i64, m: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 1 {
        mod_pow(b * b % m, exp >> 1, m) * b % m
    } else {
        mod_pow(b * b % m, exp >> 1, m)
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
        assert_eq!(sum_and_multiply("1000", &[[0, 3], [1, 1]]), [1, 0]);
        assert_eq!(sum_and_multiply("9876543210", &[[0, 9]]), [444444137]);
    }

    #[test]
    fn test() {}
}
