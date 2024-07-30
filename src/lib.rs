pub fn minimum_deletions(s: &str) -> i32 {
    let mut stack = vec![];
    let mut res = 0;
    for ch in s.chars() {
        if stack.last().is_some_and(|&c| c == 'b') && ch == 'a' {
            // number of "ba" pairs == number of deletions
            stack.pop();
            res += 1;
        } else {
            stack.push(ch)
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(minimum_deletions("aababbab"), 2);
        debug_assert_eq!(minimum_deletions("bbaaaaabb"), 2);
        // debug_assert_eq!(trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]), 6);
        // debug_assert_eq!(trap(vec![4, 2, 0, 3, 2, 5]), 9);
    }

    #[test]
    fn test() {}
}
