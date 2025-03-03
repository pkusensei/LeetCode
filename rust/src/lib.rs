mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn sort_even_odd(nums: Vec<i32>) -> Vec<i32> {
    let [mut odds, mut evens] = [vec![], vec![]];
    for (i, &v) in nums.iter().enumerate() {
        if i & 1 == 1 {
            odds.push(v);
        } else {
            evens.push(v);
        }
    }
    odds.sort_unstable_by(|a, b| b.cmp(a));
    evens.sort_unstable();
    let mut res: Vec<_> = evens
        .iter()
        .zip(odds.iter())
        .flat_map(|(&a, &b)| [a, b])
        .collect();
    if evens.len() > odds.len() {
        res.push(*evens.last().unwrap());
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
