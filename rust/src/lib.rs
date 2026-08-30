mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sum_decoded(nums: Vec<i64>) -> i32 {
    nums.iter()
        .map(|num| {
            let width = num % 10;
            let d = (num / 10).to_string();
            let x = d[..width as usize].parse::<i64>().unwrap();
            let y = d[width as usize..].parse::<i64>().unwrap();
            mod_pow(x, y)
        })
        .fold(0, |acc, v| (acc + v) % M) as i32
}

const M: i64 = 1_000_000_007;
const fn mod_pow(base: i64, exp: i64) -> i64 {
    if exp == 0 {
        return 1;
    }
    if exp & 1 == 0 {
        mod_pow(base * base % M, exp >> 1)
    } else {
        mod_pow(base * base % M, exp >> 1) * base % M
    }
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
