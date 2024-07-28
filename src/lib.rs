use std::collections::HashMap;

pub fn find_substring(s: &str, words: &[&str]) -> Vec<i32> {
    let count = words.len();
    let unit_len = words.first().map(|s| s.len()).unwrap_or_default();
    let sub_len = count * unit_len;
    if !(1..=s.len()).contains(&sub_len) {
        return vec![];
    }

    let subs: HashMap<&[u8], i32> = words.iter().fold(HashMap::new(), |mut acc, word| {
        *acc.entry(word.as_bytes()).or_insert(0) += 1;
        acc
    });
    s.as_bytes()
        .windows(sub_len)
        .enumerate()
        .filter_map(|(idx, w)| {
            let sub: HashMap<&[u8], i32> =
                w.chunks(unit_len).fold(HashMap::new(), |mut acc, chunk| {
                    *acc.entry(chunk).or_insert(0) += 1;
                    acc
                });
            if subs == sub {
                Some(idx as i32)
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(
            find_substring("barfoothefoobarman", &["foo", "bar"]),
            [0, 9]
        );
        debug_assert_eq!(
            find_substring(
                "wordgoodgoodgoodbestword",
                &["word", "good", "best", "word"]
            ),
            []
        );
        debug_assert_eq!(
            find_substring("barfoofoobarthefoobarman", &["bar", "foo", "the"]),
            [6, 9, 12]
        )
    }

    #[test]
    fn test() {}
}
