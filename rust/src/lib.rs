mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn array_change(nums: Vec<i32>, operations: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;
    let n = nums.len();
    let mut map = nums
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, &num)| {
            acc.insert(num, i);
            acc
        });
    for op in operations.iter() {
        let [a, b] = op[..] else { unreachable!() };
        let i = map.remove(&a).unwrap();
        map.insert(b, i);
    }
    let mut res = vec![0; n];
    for (num, i) in map.into_iter() {
        res[i] = num;
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
