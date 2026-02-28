mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_of_numbers(l: i32, r: i32, k: i32) -> i32 {
    let [l, r, k] = [l, r, k].map(i64::from);
    let count = r + 1 - l;
    let sum = (l + r) * count / 2;
    // total_number = pow(count, k)
    // For each digit, d_count = pow(count, k)/count
    let d_count = mod_pow(count, k - 1);
    // For each digit position, digit_sum = sum * d_count * pow(10, p)
    // total_sum = (sum * d_count)* (pow(10, 0)+pow(10, 1)+...pow(10, k-1))
    //           = sum * d_count * (pow(10, k) - 1) / 9
    let pow10 = mod_pow(10, k);
    let gsum = (pow10 - 1) * mod_pow(9, M - 2) % M;
    (sum * d_count % M * gsum % M) as i32
}

const M: i64 = 1_000_000_007;

const fn mod_pow(b: i64, p: i64) -> i64 {
    if p == 0 {
        return 1;
    }
    if p & 1 == 0 {
        mod_pow(b * b % M, p >> 1)
    } else {
        mod_pow(b * b % M, p >> 1) * b % M
    }
}

// pow 0,1,2
// 1 1 1
// 1 1 2
// 1 2 1
// 1 2 2
// 2 1 1
// 2 2 1

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
        assert_eq!(sum_of_numbers(1, 2, 2), 66);
        assert_eq!(sum_of_numbers(0, 1, 3), 444);
        assert_eq!(sum_of_numbers(5, 5, 10), 555555520);
    }

    #[test]
    fn test() {
        assert_eq!(sum_of_numbers(5, 5, 8299924), 170388343);
    }
}
