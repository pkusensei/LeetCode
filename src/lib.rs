use std::collections::HashSet;

pub fn permute_unique(nums: &[i32]) -> Vec<Vec<i32>> {
    match nums {
        [head] => vec![vec![*head]],
        [head, tail @ ..] => {
            let mut res = HashSet::new();
            for mut candid in permute_unique(tail) {
                for i in 0..candid.len() {
                    let mut v = candid.clone();
                    v.insert(i, *head);
                    res.insert(v);
                }
                candid.push(*head);
                res.insert(candid);
            }
            res.into_iter().collect()
        }
        [] => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(
        //     permute(&[1, 2, 3]),
        //     vec![
        //         vec![1, 2, 3],
        //         vec![1, 3, 2],
        //         vec![2, 1, 3],
        //         vec![2, 3, 1],
        //         vec![3, 1, 2],
        //         vec![3, 2, 1]
        //     ]
        // );
        debug_assert_eq!(
            permute_unique(&[1, 1, 2]),
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]]
        );
    }

    #[test]
    fn test() {}
}
