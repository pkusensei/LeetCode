mod helper;

use std::collections::HashMap;

#[allow(unused_imports)]
use helper::*;

pub fn is_isomorphic(s: &str, t: &str) -> bool {
    if s.len() < 2 {
        return true;
    }

    let mut m1 = HashMap::new();
    let mut m2 = HashMap::new();
    for (b1, b2) in s.bytes().zip(t.bytes()) {
        if m1.get(&b1).is_some_and(|&seen| seen != b2) {
            return false;
        }
        if m2.get(&b2).is_some_and(|&seen| seen != b1) {
            return false;
        }
        m1.insert(b1, b2);
        m2.insert(b2, b1);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_isomorphic("egg", "add"));
        debug_assert!(!is_isomorphic("foo", "bar"));
        debug_assert!(is_isomorphic("paper", "title"));
    }

    #[test]
    fn test() {
        debug_assert!(!is_isomorphic("badc", "baba"));
    }
}
