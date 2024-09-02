mod helper;

#[allow(unused_imports)]
use helper::*;

pub fn is_anagram(s: &str, t: &str) -> bool {
    s.len() == t.len()
        && s.bytes()
            .zip(t.bytes())
            .fold(vec![0; 26], |mut count, (b1, b2)| {
                count[usize::from(b1 - b'a')] += 1;
                count[usize::from(b2 - b'a')] -= 1;
                count
            })
            .into_iter()
            .all(|c| c == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_anagram("anagram", "nagaram"));
        debug_assert!(!is_anagram("rat", "car"));
    }

    #[test]
    fn test() {}
}
