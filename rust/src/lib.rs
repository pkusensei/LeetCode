mod dsu;
mod fenwick_tree;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn assign_elements(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let map = elements
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, &v)| {
            acc.entry(v).or_insert(i as i32);
            acc
        });
    groups
        .iter()
        .map(|&num| find(num, &map).unwrap_or(-1))
        .collect()
}

fn find(num: i32, map: &HashMap<i32, i32>) -> Option<i32> {
    let mut res = i32::MAX;
    for v in 1..=num.isqrt() {
        if num % v > 0 {
            continue;
        }
        if let Some(&i) = map.get(&v) {
            res = res.min(i)
        }
        if let Some(&i) = map.get(&(num / v)) {
            res = res.min(i)
        }
    }
    if res == i32::MAX { None } else { Some(res) }
}

pub fn with_sieve(groups: &[i32], elements: &[i32]) -> Vec<i32> {
    let max = *groups.iter().max().unwrap();
    let mut sieve = vec![-1; 1 + max as usize];
    for (idx, &v) in elements.iter().enumerate() {
        if v > max || sieve[v as usize] > -1 {
            continue;
        }
        let mut curr = v;
        while curr <= max {
            if sieve[curr as usize] == -1 {
                sieve[curr as usize] = idx as i32;
            }
            curr += v;
        }
    }
    groups.iter().map(|&v| sieve[v as usize]).collect()
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
        assert_eq!(assign_elements(&[8, 4, 3, 2, 4], &[4, 2]), [0, 0, -1, 1, 0]);
        assert_eq!(assign_elements(&[2, 3, 5, 7], &[5, 3, 3]), [-1, 1, 0, -1]);
        assert_eq!(assign_elements(&[10, 21, 30, 41], &[2, 1]), [0, 1, 0, 1]);

        assert_eq!(with_sieve(&[8, 4, 3, 2, 4], &[4, 2]), [0, 0, -1, 1, 0]);
        assert_eq!(with_sieve(&[2, 3, 5, 7], &[5, 3, 3]), [-1, 1, 0, -1]);
        assert_eq!(with_sieve(&[10, 21, 30, 41], &[2, 1]), [0, 1, 0, 1]);
    }

    #[test]
    fn test() {}
}
