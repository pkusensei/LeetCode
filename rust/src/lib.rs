mod dsu;
mod helper;
mod trie;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn find_original_array(changed: &mut [i32]) -> Vec<i32> {
    let n = changed.len();
    if n & 1 == 1 {
        return vec![];
    }
    let mut nums = changed.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });
    changed.sort_unstable();
    let n = n / 2;
    let mut res = Vec::with_capacity(n);
    let mut idx = 0;
    while res.len() < n && idx < 2 * n {
        let curr = changed[idx];
        if curr == 0 {
            let Some(v) = nums.get_mut(&0) else {
                return vec![];
            };
            if (*v) & 1 == 1 {
                return vec![];
            }
            if *v >= 2 {
                *v -= 2;
                res.push(0);
            }
        } else if nums.get(&curr).is_some_and(|v| *v > 0)
            && nums.get(&(2 * curr)).is_some_and(|v| *v > 0)
        {
            res.push(curr);
            nums.entry(curr).and_modify(|v| *v -= 1);
            nums.entry(2 * curr).and_modify(|v| *v -= 1);
        }
        idx += 1;
    }
    if res.len() == n {
        res
    } else {
        vec![]
    }
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
        assert_eq!(find_original_array(&mut [1, 3, 4, 2, 6, 8]), [1, 3, 4]);
        assert!(find_original_array(&mut [6, 3, 0, 1]).is_empty());
        assert!(find_original_array(&mut [1]).is_empty());
    }

    #[test]
    fn test() {}
}
