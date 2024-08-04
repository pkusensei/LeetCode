pub fn subsets(nums: &[i32]) -> Vec<Vec<i32>> {
    solve(nums)
}

fn solve(nums: &[i32]) -> Vec<Vec<i32>> {
    match nums {
        [head] => vec![vec![], vec![*head]],
        [head, tail @ ..] if !tail.is_empty() => {
            let mut res = vec![];
            for mut v in solve(tail) {
                res.push(v.clone());
                v.push(*head);
                res.push(v)
            }
            res
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(
        //     subsets(&[1, 2, 3]),
        //     [
        //         vec![],
        //         vec![1],
        //         vec![2],
        //         vec![1, 2],
        //         vec![3],
        //         vec![1, 3],
        //         vec![2, 3],
        //         vec![1, 2, 3]
        //     ]
        // );
        debug_assert_eq!(subsets(&[0]), [vec![], vec![0]])
    }

    #[test]
    fn test() {}
}
