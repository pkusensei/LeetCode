pub fn kth_distinct(arr: &[&str], k: i32) -> String {
    let mut strs: Vec<_> = arr
        .iter()
        .enumerate()
        .fold(std::collections::HashMap::new(), |mut acc, (idx, s)| {
            acc.entry(s).or_insert((idx, 0)).1 += 1;
            acc
        })
        .into_iter()
        .filter_map(
            |(s, (idx, count))| {
                if count == 1 {
                    Some((s, idx))
                } else {
                    None
                }
            },
        )
        .collect();
    strs.sort_unstable_by_key(|(_s, i)| *i);
    strs.get(k as usize - 1)
        .map(|(s, _i)| s.to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basics() {
        debug_assert_eq!(kth_distinct(&["d", "b", "c", "b", "c", "a"], 2), "a");
        debug_assert_eq!(kth_distinct(&["aaa", "aa", "a"], 1), "aaa");
        debug_assert_eq!(kth_distinct(&["a", "b", "a"], 3), "");
    }

    #[test]
    fn test() {}
}
