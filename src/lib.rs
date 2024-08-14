mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn reverse_words(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(reverse_words("the sky is blue"), "blue is sky the");
        debug_assert_eq!(reverse_words("  hello world  "), "world hello");
        debug_assert_eq!(reverse_words("a good   example"), "example good a");
    }

    #[test]
    fn test() {}
}
