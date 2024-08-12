pub fn partition(s: &str) -> Vec<Vec<String>> {
    solve(s, 0)
        .into_iter()
        .map(|v| v.into_iter().rev().map(|s| s.to_string()).collect())
        .collect()
}

fn solve(s: &str, start: usize) -> Vec<Vec<&str>> {
    if start + 1 == s.len() {
        return vec![vec![&s[start..]]];
    }

    let mut res = vec![];
    for end in start + 1..s.len() {
        let curr = &s[start..end];
        if is_palindrome(curr) {
            let temp = solve(s, end).into_iter().map(|mut v| {
                v.push(curr);
                v
            });
            res.extend(temp);
        }
    }
    if is_palindrome(&s[start..]) {
        res.push(vec![&s[start..]])
    }
    res
}

fn is_palindrome(s: &str) -> bool {
    if s.len() < 2 {
        return true;
    }
    s.bytes()
        .rev()
        .zip(s.bytes().take(s.len() / 2 + 1))
        .all(|(b1, b2)| b1 == b2)
}

// type Coord = (usize, usize);

// fn neighbors((a, b): Coord) -> impl Iterator<Item = Coord> {
//     [
//         (a.saturating_sub(1), b),
//         (a + 1, b),
//         (a, b.saturating_sub(1)),
//         (a, b + 1),
//     ]
//     .into_iter()
//     .filter(move |&p| p != (a, b))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(partition("aab"), [vec!["a", "a", "b"], vec!["aa", "b"]]);
        debug_assert_eq!(partition("a"), [["a"]]);
    }

    #[test]
    fn test() {
        debug_assert_eq!(partition("bb"), [vec!["b", "b"], vec!["bb"]]);
        debug_assert_eq!(partition("cdd"), [vec!["c", "d", "d"], vec!["c", "dd"]]);
    }
}
