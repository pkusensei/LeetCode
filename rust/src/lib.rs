mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_products_of_elements(queries: &[[i64; 3]]) -> Vec<i32> {
    queries
        .iter()
        .map(|q| mod_pow(2, query(q[1]) - query(q[0] - 1), q[2]) as i32)
        .collect()
}

// sum of all bits of [1..=num]
fn count(num: i64) -> i64 {
    if num == 0 {
        return 0;
    }
    // 10_i32 0b_1010 bit == 3
    let bit = i64::from(num.ilog2());
    // base = 0b_1000
    let base = 1 << bit;
    let res = bit * (base >> 1);
    res + count(num - base) + num - base
}

fn mul(num: i64) -> i64 {
    if num == 0 {
        return 0;
    }
    let bit = i64::from(num.ilog2());
    let base = 1 << bit;
    let res = ((bit - 1) * bit * base) >> 2;
    res + mul(num - base) + bit * (num - base)
}

fn query(mut num: i64) -> i64 {
    if num < 0 {
        return 0;
    }
    num += 1;
    let mut left = 1;
    let mut right = 10_i64.pow(15);
    while left < right {
        let mid = left + (right - left) / 2;
        if count(mid) < num {
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    let mut x = left - 1;
    let remain = num - count(x);
    let mut res = mul(x);
    for _ in 0..remain {
        let bit = x & -x;
        res += i64::from(bit.ilog2());
        x -= bit;
    }
    res
}

const fn mod_pow(base: i64, exp: i64, m: i64) -> i64 {
    if m == 1 {
        return 0;
    }
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(base * base % m, exp >> 1, m)
    } else {
        mod_pow(base * base % m, exp >> 1, m) * base % m
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
        assert_eq!(find_products_of_elements(&[[1, 3, 7]]), [4]);
        assert_eq!(find_products_of_elements(&[[2, 5, 3], [7, 7, 4]]), [2, 2]);
    }

    #[test]
    fn test() {
        assert_eq!(find_products_of_elements(&[[0, 2, 11]]), [2]);
        assert_eq!(find_products_of_elements(&[[9, 9, 1]]), [0]);
    }
}
