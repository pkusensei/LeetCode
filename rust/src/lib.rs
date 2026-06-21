mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_total_value(value: &[i32], decay: &[i32], m: i32) -> i32 {
    const M: i64 = 1_000_000_007;

    let m = i64::from(m);
    let max = i64::from(*value.iter().max().unwrap());
    let mut left = 1;
    let mut right = max;
    while left < right {
        let mid = left + (1 + right - left) / 2;
        if find(&value, &decay, mid) >= m {
            left = mid;
        } else {
            right = mid - 1;
        }
    }
    let mut res = 0;
    for (&val, &dec) in value.iter().zip(decay.iter()) {
        let [val, dec] = [val, dec].map(i64::from);
        if val >= left {
            let count = 1 + (val - left).max(0) / dec;
            let last = val - dec * (count - 1);
            res += i64::from(val + last) * i64::from(count) / 2 % M;
            res %= M;
        }
    }
    let f = find(&value, &decay, left);
    res -= i64::from(f - m).max(0) * i64::from(left);
    res.rem_euclid(M) as i32
}

fn find(value: &[i32], decay: &[i32], mid: i64) -> i64 {
    let mut res = 0;
    for (&val, &dec) in value.iter().zip(decay) {
        let [val, dec] = [val, dec].map(i64::from);
        if val >= mid {
            res += 1;
            res += (val - mid) / dec
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
        assert_eq!(max_total_value(&[6, 5, 4], &[2, 1, 1], 4), 19);
    }

    #[test]
    fn test() {}
}
