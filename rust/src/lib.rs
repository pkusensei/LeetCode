mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn replace_non_coprimes(nums: &[i32]) -> Vec<i32> {
    let mut memo = HashMap::new();
    let mut res = vec![];
    for &num in nums.iter() {
        let mut num = i64::from(num);
        while res.last().is_some_and(|&v| gcd(v, num, &mut memo) > 1) {
            let v = res.pop().unwrap();
            let lcm = v * num / gcd(v, num, &mut memo);
            num = lcm;
        }
        res.push(num);
    }
    res.into_iter().map(|v| v as i32).collect()
}

fn gcd(a: i64, b: i64, memo: &mut HashMap<[i64; 2], i64>) -> i64 {
    if let Some(&v) = memo.get(&[a, b]) {
        return v;
    }
    let res = if a == 0 { b } else { gcd(b % a, a, memo) };
    memo.insert([a, b], res);
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
        assert_eq!(replace_non_coprimes(&[6, 4, 3, 2, 7, 6, 2]), [12, 7, 6]);
        assert_eq!(replace_non_coprimes(&[2, 2, 1, 1, 3, 3, 3]), [2, 1, 1, 3]);
    }

    #[test]
    fn test() {
        assert_eq!(
            replace_non_coprimes(&[287, 41, 49, 287, 899, 23, 23, 20677, 5, 825]),
            [2009, 20677, 825]
        );
    }
}
