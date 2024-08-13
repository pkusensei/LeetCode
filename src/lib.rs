pub fn word_break(s: &str, word_dict: &[&str]) -> bool {
    let size = s.len();
    let mut dp = vec![false; size + 1];
    solve(&mut dp, s, word_dict, 0, &mut vec![None; size + 1]);
    *dp.last().unwrap()
}

fn solve(
    dp: &mut [bool],
    s: &str,
    word_dict: &[&str],
    start: usize,
    seen: &mut [Option<bool>],
) -> bool {
    if start == s.len() {
        return true;
    }
    if let Some(b) = seen[start] {
        return b;
    }
    for word in word_dict {
        if s[start..].strip_prefix(word).is_some() {
            dp[start + word.len()] |= solve(dp, s, word_dict, start + word.len(), seen);
        }
    }
    seen[start] = Some(dp[start]);
    dp[start]
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
        debug_assert!(word_break("leetcode", &["leet", "code"]));
        debug_assert!(word_break("applepenapple", &["apple", "pen"]));
        debug_assert!(!word_break(
            "catsandog",
            &["cats", "dog", "sand", "and", "cat"]
        ));
    }

    #[test]
    fn test() {
        debug_assert!(!word_break("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab", 
        &["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"]));
    }
}
