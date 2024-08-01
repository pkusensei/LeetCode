pub fn permute(nums: &[i32]) -> Vec<Vec<i32>> {
    match nums {
        [head] => vec![vec![*head]],
        [head, tail @ ..] => {
            let mut res = vec![];
            for mut candid in permute(tail) {
                for i in 0..candid.len() {
                    let mut v = candid.clone();
                    v.insert(i, *head);
                    res.push(v);
                }
                candid.push(*head);
                res.push(candid)
            }
            res
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
        debug_assert_eq!(permute(&[0, 1]), vec![vec![0, 1], vec![1, 0]]);
        debug_assert_eq!(permute(&[1]), vec![vec![1]]);
    }

    #[test]
    fn test() {}
}
