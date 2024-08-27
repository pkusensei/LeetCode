mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn shortest_palindrome(s: &str) -> String {
    fn kmp(s: &str) -> String {
        let rev: Vec<_> = s.chars().rev().collect();
        let new_s: Vec<_> = s
            .chars()
            .chain(std::iter::once('#'))
            .chain(rev.iter().copied())
            .collect();
        let size = new_s.len();
        let mut prefix = vec![0; size];
        for i in 1..size {
            let mut t = prefix[i - 1];
            while t > 0 && new_s[t] != new_s[i] {
                t = prefix[t - 1]
            }
            if new_s[t] == new_s[i] {
                t += 1
            }
            prefix[i] = t
        }
        rev.into_iter()
            .take(s.len() - prefix[size - 1])
            .chain(s.chars())
            .collect()
    }
    // kmp(s)
    solve(&s.chars().collect::<Vec<_>>())
}

fn solve(s: &[char]) -> String {
    let size = s.len();
    let mut i = 0;
    for &b in s.iter().rev() {
        if s[i] == b {
            i += 1;
        }
    }
    if i == size {
        return s.iter().collect();
    }
    let remain: String = s[i..].iter().rev().collect();
    format!(
        "{}{}{}",
        remain,
        solve(&s[..i]),
        s.iter().skip(i).collect::<String>()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        // debug_assert_eq!(shortest_palindrome("aacecaaa"), "aaacecaaa");
        debug_assert_eq!(shortest_palindrome("abcd"), "dcbabcd");
    }

    #[test]
    fn test() {}
}
