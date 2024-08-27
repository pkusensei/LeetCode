mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let nums: Vec<_> = (1..=9).collect();
        solve(&nums, k, n)
}

fn solve(nums: &[i32], k: i32, n: i32) -> Vec<Vec<i32>> {
    match nums {
        [] if n == 0 && k == 0 => vec![vec![]],
        [] => vec![],
        [head, ..] if *head == n && k == 1 => vec![vec![n]],
        [_] => vec![],
        [head, tail @ ..] => {
            let mut res = solve(tail, k, n);
            for mut v in solve(tail, k - 1, n - head) {
                v.push(*head);
                res.push(v);
            }
            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(combination_sum3(3, 7), [[1, 2, 4]]);
        // debug_assert_eq!(combination_sum3(3, 9), [[1, 2, 6], [1, 3, 5], [2, 3, 4]]);
        debug_assert!(combination_sum3(4, 1).is_empty());
    }

    #[test]
    fn test() {}
}
