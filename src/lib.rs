use std::collections::HashMap;

pub fn min_window(s: &str, t: &str) -> String {
    let (s1, s2) = (s.len(), t.len());
    if s1 < s2 {
        return String::new();
    }
    if s == t {
        return s.to_string();
    }

    let needle = t.bytes().fold(HashMap::new(), |mut acc, b| {
        *acc.entry(b).or_insert(0) += 1;
        acc
    });
    let hay = s.as_bytes();
    let mut pairs = vec![];
    let (mut left, mut right) = (0, t.len());
    let mut window = hay[left..right].iter().fold(HashMap::new(), |mut acc, &b| {
        if needle.contains_key(&b) {
            *acc.entry(b).or_insert(0) += 1;
        }
        acc
    });
    right = expand_right(&mut window, &needle, right, hay);
    if contains(&window, &needle) {
        pairs.push((left, right));
    }
    while left + t.len() < right {
        if needle.contains_key(&hay[left]) {
            if let Some(v) = window.get_mut(&hay[left]) {
                *v -= 1
            }
            left += 1;
            // if contains == true, expand_right does nothing
            right = expand_right(&mut window, &needle, right, hay);
            if contains(&window, &needle) {
                pairs.push((left, right));
            }
        } else {
            left += 1;
            if contains(&window, &needle) {
                pairs.push((left, right));
            }
        }
    }
    pairs
        .into_iter()
        .min_by_key(|(l, r)| r - l)
        .map(|(l, r)| s[l..r].to_string())
        .unwrap_or_default()
}

fn expand_right(
    window: &mut HashMap<u8, i32>,
    needle: &HashMap<u8, i32>,
    mut right: usize,
    hay: &[u8],
) -> usize {
    while !contains(window, needle) && right < hay.len() {
        if needle.contains_key(&hay[right]) {
            *window.entry(hay[right]).or_insert(0) += 1;
        }
        right += 1
    }
    right
}

fn contains(window: &HashMap<u8, i32>, needle: &HashMap<u8, i32>) -> bool {
    window.len() == needle.len()
        && needle
            .iter()
            .all(|(k, v)| window.get(k).is_some_and(|count| count >= v))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(min_window("ADOBECODEBANC", "ABC"), "BANC");
        debug_assert_eq!(min_window("a", "a"), "a");
        debug_assert_eq!(min_window("a", "aa"), "");
    }

    #[test]
    fn test() {
        debug_assert_eq!(min_window("ab", "A"), "")
    }
}
