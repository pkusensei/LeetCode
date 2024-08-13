pub fn word_break(s: &str, word_dict: &[&str]) -> Vec<String> {
    let mut res = vec![];
    solve(s, word_dict, 0, &mut res, &mut vec![]);
    res.into_iter().map(|v| v.join(" ")).collect()
}

fn solve<'a>(
    s: &str,
    word_dict: &[&'a str],
    start: usize,
    res: &mut Vec<Vec<&'a str>>,
    curr: &mut Vec<&'a str>,
) {
    if start == s.len() {
        res.push(curr.clone());
    }
    for word in word_dict {
        if s[start..].strip_prefix(word).is_some() {
            curr.push(word);
            solve(s, word_dict, start + word.len(), res, curr);
            curr.pop();
        }
    }
}

// fn is_palindrome(s: &str) -> bool {
//     if s.len() < 2 {
//         return true;
//     }
//     s.bytes()
//         .rev()
//         .zip(s.bytes().take(s.len() / 2 + 1))
//         .all(|(b1, b2)| b1 == b2)
// }

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
        debug_assert_eq!(
            word_break("catsanddog", &["cats", "dog", "sand", "and", "cat"]),
            ["cats and dog", "cat sand dog"]
        );
        debug_assert_eq!(
            word_break("catsandog", &["cats", "dog", "sand", "and", "cat"]),
            [""; 0]
        );
        debug_assert_eq!(
            word_break(
                "pineapplepenapple",
                &["apple", "pen", "applepen", "pine", "pineapple"]
            ),
            [
                "pine apple pen apple",
                "pineapple pen apple",
                "pine applepen apple"
            ]
        );
    }

    #[test]
    fn test() {}
}
