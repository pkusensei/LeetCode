mod binary_lifting;
mod dsu;
mod fenwick_tree;
mod helper;
mod matrix;
mod seg_tree;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn first_unique_freq(nums: Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let freq = nums.iter().fold(vec![0; 100_001], |mut acc, &num| {
        acc[num as usize] += 1;
        acc
    });
    let map = freq
        .iter()
        .enumerate()
        .filter_map(|(num, &f)| if f > 0 { Some((num, f)) } else { None })
        .fold(HashMap::<_, Vec<_>>::new(), |mut acc, (num, f)| {
            acc.entry(f).or_default().push(num);
            acc
        });
    for num in nums {
        let f = freq[num as usize];
        if map[&f].len() == 1 {
            return num;
        }
    }
    -1
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
    fn basics() {}

    #[test]
    fn test() {}
}
