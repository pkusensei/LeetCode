mod dsu;
mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn split_into_fibonacci(num: &str) -> Vec<i32> {
    let mut res = vec![];
    backtrack(num, 0, &mut res).unwrap_or_default()
}

fn backtrack(num: &str, idx: usize, curr: &mut Vec<i32>) -> Option<Vec<i32>> {
    if idx == num.len() {
        if curr.len() > 2 {
            return Some(curr.clone());
        }
        return None;
    }
    if num[idx..].starts_with('0') {
        let n = curr.len();
        if n >= 2 {
            let (n1, n2) = (curr[n - 1], curr[n - 2]);
            if Some(0) == n1.checked_add(n2) {
                curr.push(0);
                return backtrack(num, 1 + idx, curr);
            }
            return None;
        } else {
            curr.push(0);
            if let Some(v) = backtrack(num, 1 + idx, curr) {
                return Some(v);
            }
            curr.pop();
            return None;
        }
    }
    for i in 1 + idx..=num.len() {
        let Ok(temp) = num[idx..i].parse::<i32>() else {
            continue;
        };
        let n = curr.len();
        if n >= 2 {
            let (n1, n2) = (curr[n - 1], curr[n - 2]);
            if Some(temp) == n1.checked_add(n2) {
                curr.push(temp);
                if let Some(v) = backtrack(num, i, curr) {
                    return Some(v);
                }
                curr.pop();
            }
        } else {
            curr.push(temp);
            if let Some(v) = backtrack(num, i, curr) {
                return Some(v);
            }
            curr.pop();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(split_into_fibonacci("1101111"), [11, 0, 11, 11]);
        debug_assert!(split_into_fibonacci("112358130").is_empty());
        debug_assert!(split_into_fibonacci("0123").is_empty());
    }

    #[test]
    fn test() {
        debug_assert_eq!(split_into_fibonacci("0000"), [0, 0, 0, 0]);
        debug_assert_eq!(
            split_into_fibonacci("1320581321313221264343965566089105744171833277577"),
            [13205, 8, 13213, 13221, 26434, 39655, 66089, 105744, 171833, 277577]
        );
    }

    #[allow(dead_code)]
    fn sort_eq<T1, T2, I1, I2>(mut i1: I1, mut i2: I2)
    where
        T1: Ord + Debug + PartialEq<T2>,
        T2: Ord + Debug + PartialEq<T1>,
        I1: AsMut<[T1]>,
        I2: AsMut<[T2]>,
    {
        i1.as_mut().sort_unstable();
        i2.as_mut().sort_unstable();
        debug_assert_eq!(i1.as_mut(), i2.as_mut());
    }
}
