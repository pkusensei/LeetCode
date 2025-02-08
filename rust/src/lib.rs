mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    let max = *costs.iter().max().unwrap();
    let mut buckets = vec![0; 1 + max as usize];
    for &c in costs.iter() {
        buckets[c as usize] += 1;
    }
    let mut num = coins;
    let mut res = 0;
    for (c, &count) in buckets.iter().enumerate() {
        let c = c as i32;
        if num > c * count {
            res += count;
            num -= c * count;
        } else {
            res += num / c;
            break;
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
    fn basics() {}

    #[test]
    fn test() {}
}
