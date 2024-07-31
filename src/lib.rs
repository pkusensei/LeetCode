pub fn is_match(s: &str, p: &str) -> bool {
    let haystack: Vec<_> = s.chars().collect();
    let pattern: Vec<_> = p.chars().collect();
    solve(&haystack, &pattern, 0, 0, None, 0)
}

fn solve(
    haystack: &[char],
    pattern: &[char],
    h_idx: usize,
    p_idx: usize,
    last_star: Option<usize>,
    last_match: usize,
) -> bool {
    if h_idx == haystack.len() {
        return pattern[p_idx..].iter().all(|&c| c == '*');
    }

    if h_idx < haystack.len() {
        if pattern
            .get(p_idx)
            .is_some_and(|&p| p == '?' || haystack[h_idx] == p)
        {
            return solve(
                haystack,
                pattern,
                h_idx + 1,
                p_idx + 1,
                last_star,
                last_match,
            );
        } else if pattern.get(p_idx).is_some_and(|&p| p == '*') {
            return solve(haystack, pattern, h_idx, p_idx + 1, Some(p_idx), h_idx);
        } else if let Some(star) = last_star {
            return solve(
                haystack,
                pattern,
                last_match,
                star + 1,
                last_star,
                last_match + 1,
            );
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(is_match("aa", "a"), false);
        debug_assert_eq!(is_match("aa", "*"), true);
        debug_assert_eq!(is_match("cb", "?a"), false)
    }

    #[test]
    fn test() {}
}
