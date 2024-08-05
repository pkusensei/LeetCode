use std::collections::HashMap;

pub fn is_scramble(s1: &str, s2: &str) -> bool {
    dfs(s1, s2, &mut HashMap::new())
}

fn dfs<'a>(s1: &'a str, s2: &'a str, seen: &mut HashMap<(&'a str, &'a str), bool>) -> bool {
    if let Some(&res) = seen.get(&(s1, s2)) {
        return res;
    }
    if s1.len() != s2.len() {
        seen.insert((s1, s2), false);
        return false;
    }
    if s1 == s2 {
        seen.insert((s1, s2), true);
        return true;
    }
    if !is_anagram(s1, s2) {
        seen.insert((s1, s2), false);
        return false;
    }

    let size = s1.len();
    for length in 1..size {
        let res = (dfs(&s1[..length], &s2[..length], seen)
            && dfs(&s1[length..], &s2[length..], seen))
            || (dfs(&s1[..length], &s2[size - length..], seen)
                && dfs(&s1[length..], &s2[..size - length], seen));
        if res {
            seen.insert((s1, s2), true);
            return true;
        }
    }

    seen.insert((s1, s2), false);
    false
}

fn is_anagram(s1: &str, s2: &str) -> bool {
    let mut count = [0; 26];
    for b in s1.bytes() {
        count[(b - b'a') as usize] += 1;
    }
    for b in s2.bytes() {
        count[(b - b'a') as usize] -= 1;
    }
    count.into_iter().all(|c| c == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert!(is_scramble("great", "rgeat"));
        debug_assert!(!is_scramble("abcde", "caebd"));
        debug_assert!(is_scramble("a", "a"));
    }

    #[test]
    fn test() {}
}
