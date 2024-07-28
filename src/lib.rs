pub fn generate_parenthesis(n: i32) -> Vec<String> {
    solve(n, 1, 0)
        .into_iter()
        .map(|s| format!("({s}"))
        .filter(|s| s.len() == n as usize * 2)
        .collect()
}

fn solve(n: i32, left: i32, right: i32) -> Vec<String> {
    if left == n && right < n {
        solve(n, left, right + 1)
            .into_iter()
            .map(|s| format!("){s}"))
            .collect()
    } else if left < n && right < n && left >= right {
        solve(n, left + 1, right)
            .into_iter()
            .map(|s| format!("({s}"))
            .chain(
                solve(n, left, right + 1)
                    .into_iter()
                    .map(|s| format!("){s}")),
            )
            .collect()
    } else {
        vec!["".to_string()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        {
            let mut ans = generate_parenthesis(3);
            ans.sort_unstable();
            debug_assert_eq!(ans, ["((()))", "(()())", "(())()", "()(())", "()()()"])
        }
        {
            let mut ans = generate_parenthesis(1);
            ans.sort_unstable();
            debug_assert_eq!(ans, ["()"])
        }
    }

    #[test]
    fn test() {}
}
