pub fn is_palindrome(s: &str) -> bool {
    let it = s.bytes().filter_map(|b| {
        if b.is_ascii_alphanumeric() {
            Some(b.to_ascii_lowercase())
        } else {
            None
        }
    });
    it.clone()
        .rev()
        .take(s.len())
        .zip(it.take(s.len()))
        .all(|(b1, b2)| b1 == b2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_palindrome("A man, a plan, a canal: Panama"));
        debug_assert!(!is_palindrome("race a car"));
        debug_assert!(is_palindrome(" "));
    }

    #[test]
    fn test() {}
}
