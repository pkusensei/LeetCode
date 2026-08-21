mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_kth_smallest(mut coins: Vec<i32>, k: i32) -> i64 {
    let mut arr = vec![];
    coins.sort_unstable();
    for big in coins.iter() {
        if arr.iter().all(|small| big % small > 0) {
            arr.push(*big);
        }
    }
    let coins = arr; // shadows
    let n = coins.len();
    let full = 1 << n;
    let mut lcms = Vec::with_capacity(full);
    lcms.push(0);
    for mask in 1..full {
        let lsb = mask & mask.wrapping_neg();
        let i = lsb.ilog2() as usize;
        let prev = mask ^ lsb;
        let lcm_ = if lcms[prev] == 0 {
            i64::from(coins[i])
        } else {
            lcm(lcms[prev], coins[i].into())
        };
        lcms.push(lcm_);
    }

    let mut left = i64::from(k);
    let mut right = 1 + i64::from(coins[0]) * i64::from(k);
    while left < right {
        let mid = left + (right - left) / 2;
        if count(&lcms, mid) < i64::from(k) {
            left = 1 + mid;
        } else {
            right = mid;
        }
    }
    left
}

fn count(lcms: &[i64], mid: i64) -> i64 {
    let mut res = 0;
    for (mask, lcm_) in lcms.iter().enumerate().skip(1) {
        if mask.count_ones() & 1 == 1 {
            res += mid / lcm_
        } else {
            res -= mid / lcm_
        }
    }
    res
}

const fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a > 0 {
        (a, b) = (b % a, a);
    }
    b
}
const fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

#[cfg(test)]
mod tests {

    #[allow(unused_imports)]
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
    fn basics() {}

    #[test]
    fn test() {}
}
