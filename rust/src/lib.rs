mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn destroy_targets(nums: Vec<i32>, space: i32) -> i32 {
    let map = nums.iter().fold(
        std::collections::HashMap::<_, [i32; 2]>::new(),
        |mut acc, &num| {
            let v = acc.entry(num % space).or_insert([0, i32::MAX]);
            v[0] += 1;
            v[1] = v[1].min(num);
            acc
        },
    );
    let mut count = 0;
    let mut res = i32::MAX;
    for [c, min] in map.into_values() {
        if c > count {
            count = c;
            res = min
        } else if c == count {
            res = res.min(min)
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
