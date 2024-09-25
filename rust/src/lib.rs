mod helper;
mod trie;

#[allow(unused_imports)]
use helper::*;

pub fn find_subsequences(nums: &[i32]) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut curr = vec![];
    dfs(nums, &mut curr, &mut res);
    res
}

fn dfs(nums: &[i32], curr: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
    match nums {
        [] => {
            if curr.len() > 1 {
                res.push(curr.clone());
            }
        }
        [head, tail @ ..] => match curr.last().copied() {
            None => {
                curr.push(*head);
                dfs(tail, curr, res);
                curr.pop();
                dfs(tail, curr, res);
            }
            Some(v) if v <= *head => {
                curr.push(*head);
                dfs(tail, curr, res);
                curr.pop();
                if v < *head {
                    // say curr is [1,2], head is [2]
                    // above has already checked [1,2,2]
                    // skip the non-pick case
                    dfs(tail, curr, res);
                }
            }
            _ => dfs(tail, curr, res),
        },
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    fn basics() {
        sort_eq(
            find_subsequences(&[4, 6, 7, 7]),
            [
                vec![4, 6],
                vec![4, 6, 7],
                vec![4, 6, 7, 7],
                vec![4, 7],
                vec![4, 7, 7],
                vec![6, 7],
                vec![6, 7, 7],
                vec![7, 7],
            ],
        );
        debug_assert_eq!(find_subsequences(&[4, 4, 3, 2, 1]), [[4, 4]]);
    }

    #[test]
    fn test() {}

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
